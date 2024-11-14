use std::collections::HashMap;

use atuin_client::{
    encryption,
    record::{encryption::PASETO_V4, sqlite_store::SqliteStore, store::Store},
};
use atuin_common::record::{DecryptedData, Host, HostId, Record, RecordIdx};
use ciborium_io::{Read, Write};
use ciborium_ll::{Decoder, Encoder, Header};
use eyre::{bail, ensure, eyre, Context};
use jiff::{
    tz::{Offset, TimeZone},
    Timestamp, Zoned,
};

pub use atuin_client::settings::Settings;
pub use atuin_common::record::RecordId;

pub struct Client {
    store: SqliteStore,
    host_id: HostId,
    key: [u8; 32],

    hosts: HashMap<HostId, RecordIdx>,
}

impl Client {
    pub async fn new(settings: &Settings) -> Self {
        let store = SqliteStore::new(&settings.record_store_path, settings.local_timeout)
            .await
            .unwrap();

        let host_id = atuin_client::settings::Settings::host_id().expect("failed to get host_id");
        let key = encryption::load_key(settings).unwrap().into();

        Self {
            store,
            host_id,
            key,
            hosts: HashMap::new(),
        }
    }

    pub fn test(store: SqliteStore, host_id: HostId) -> Self {
        let key = [0x55; 32];

        Self {
            store,
            host_id,
            key,
            hosts: HashMap::new(),
        }
    }

    pub async fn add_record(
        &mut self,
        note: String,
        children: Vec<RecordId>,
        datetime: Zoned,
    ) -> RecordId {
        let timestamp = datetime.timestamp().duration_since(Timestamp::UNIX_EPOCH);
        let timestamp = uuid::Timestamp::from_unix(
            uuid::NoContext,
            timestamp.as_secs() as u64,
            timestamp.subsec_nanos() as u32,
        );
        let id = RecordId(uuid::Uuid::new_v7(timestamp));

        let idx = self
            .store
            .last(self.host_id, Note::TAG)
            .await
            .unwrap()
            .map_or(0, |p| p.idx + 1);

        let note = Note {
            note,
            children,
            datetime,
        };

        let record = Record::builder()
            .id(id)
            .data(DecryptedData(note.ser_v0_to_vec()))
            .tag(Note::TAG.to_string())
            .idx(idx)
            .host(Host::new(self.host_id))
            .version(Note::VERSION.to_string())
            .build();
        let record = record.encrypt::<PASETO_V4>(&self.key);
        self.store.push(&record).await.unwrap();

        *self.hosts.entry(self.host_id).or_default() = idx + 1;

        id
    }

    pub async fn sync(&mut self, settings: &Settings) {
        atuin_client::record::sync::sync(settings, &self.store).await.unwrap();
    }

    pub async fn load_notes(
        &mut self,
        mut f: impl FnMut(HostId, RecordId, Note),
    ) -> eyre::Result<()> {
        let status = self
            .store
            .status()
            .await
            .context("loading current status")?;

        for (host_id, tags) in status.hosts {
            let Some(&last_idx) = tags.get(Note::TAG) else {
                continue;
            };
            let host_entry = self.hosts.entry(host_id).or_insert(0);
            if last_idx < *host_entry {
                continue;
            }

            loop {
                let batch = self
                    .store
                    .next(host_id, Note::TAG, *host_entry, 100)
                    .await?;
                if batch.is_empty() {
                    break;
                }

                for note_record in batch {
                    *host_entry = note_record.idx + 1;

                    if note_record.version != "v0" {
                        continue;
                    }
                    let Ok(note_record) = note_record.decrypt::<PASETO_V4>(&self.key) else {
                        continue;
                    };
                    let note = Note::deser_v0(&note_record.data.0)?;

                    f(note_record.host.id, note_record.id, note);
                }
            }
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct Note {
    pub note: String,
    pub datetime: Zoned,
    pub children: Vec<RecordId>,
}

/// <https://github.com/lucas-clemente/cbor-specs/blob/master/uuid.md>
const UUID_BYTES_TAG: u64 = 37;

impl Note {
    const TAG: &str = "note_lsm::note";
    const VERSION: &str = "v0";

    fn ser_v0_to_vec(&self) -> Vec<u8> {
        let mut buf = vec![];
        self.ser_v0(&mut buf);
        buf
    }

    fn ser_v0(&self, buf: &mut Vec<u8>) {
        self.ser_v0_inner(buf)
            .expect("encoding to vec should not fail");
    }

    fn ser_v0_inner(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let mut enc = Encoder::from(buf);
        enc.push(ciborium_ll::Header::Array(Some(3)))?;

        // note
        enc.text(&self.note, None)?;

        // datetime
        zoned_cbor_9581_ser(&self.datetime, &mut enc)?;

        // children
        enc.push(ciborium_ll::Header::Array(Some(self.children.len())))?;
        for child in &self.children {
            enc.push(ciborium_ll::Header::Tag(UUID_BYTES_TAG))?;
            enc.bytes(child.0.as_bytes(), None)?;
        }
        Ok(())
    }

    fn deser_v0(b: &[u8]) -> eyre::Result<Self> {
        let mut scratch = [0; 1024];
        let mut dec = Decoder::from(b);

        match dec.pull().map_err(map_deser_err)? {
            ciborium_ll::Header::Array(Some(3)) => {}
            header => bail!("invalid header: {header:?}"),
        };

        let note_len = match dec.pull().map_err(map_deser_err)? {
            ciborium_ll::Header::Text(len) => len,
            header => bail!("invalid header: {header:?}"),
        };

        let note = {
            let mut segments = dec.text(note_len);
            let mut note = String::with_capacity(note_len.unwrap_or_default());

            while let Some(mut segment) = segments.pull().map_err(map_deser_err)? {
                while let Some(chunk) = segment.pull(&mut scratch).map_err(map_deser_err)? {
                    note.push_str(chunk);
                }
            }

            note
        };

        let datetime = zoned_cbor_9581_deser(&mut dec)?;

        let children_len = match dec.pull().map_err(map_deser_err)? {
            ciborium_ll::Header::Array(Some(len)) => len,
            header => bail!("invalid header: {header:?}"),
        };

        let mut children = Vec::with_capacity(children_len);
        for _ in 0..children_len {
            match dec.pull().map_err(map_deser_err)? {
                ciborium_ll::Header::Tag(UUID_BYTES_TAG) => {}
                header => bail!("invalid header: {header:?}"),
            };
            match dec.pull().map_err(map_deser_err)? {
                ciborium_ll::Header::Bytes(Some(16)) => {}
                header => bail!("invalid header: {header:?}"),
            };

            let mut segments = dec.bytes(Some(16));
            let mut id = [0; 16];
            let Some(mut segment) = segments.pull().map_err(map_deser_err)? else {
                bail!("expecting 1 segment for child id")
            };
            ensure!(segment.left() == 16);
            segment.pull(&mut id[..]).map_err(map_deser_err)?;

            children.push(RecordId(uuid::Uuid::from_bytes(id)));
        }

        Ok(Self {
            note,
            children,
            datetime,
        })
    }
}

fn map_deser_err<E: std::fmt::Display>(err: ciborium_ll::Error<E>) -> eyre::Error {
    match err {
        ciborium_ll::Error::Io(io) => eyre!("io error: {io}"),
        ciborium_ll::Error::Syntax(offset) => eyre!("syntax error at byte {offset}"),
    }
}

const BASE_TIME: Header = Header::Positive(1);
const NANOSECONDS: Header = Header::Negative(!(-9i64) as u64);
const TIME_ZONE: Header = Header::Positive(10);

/// <https://www.rfc-editor.org/rfc/rfc9581.html>
fn zoned_cbor_9581_ser<E>(zdt: &Zoned, enc: &mut Encoder<impl Write<Error = E>>) -> Result<(), E> {
    let timestamp = zdt.timestamp();
    let tz = zdt.time_zone();
    let mut offset_str = String::new();

    let zone = tz.iana_name().unwrap_or_else(|| {
        use std::fmt::Write;

        // https://www.rfc-editor.org/rfc/rfc9581.html#name-keys-10-10-time-zone-hint requires hours and minutes.
        let minutes = (zdt.offset().seconds() / 60).abs();
        let (hours, minutes) = (minutes / 60, minutes % 60);
        if zdt.offset().is_negative() {
            write!(&mut offset_str, "-{hours:02}:{minutes:02}").unwrap();
        } else {
            write!(&mut offset_str, "+{hours:02}:{minutes:02}").unwrap();
        }
        &offset_str
    });

    let timestamp = timestamp.duration_since(Timestamp::UNIX_EPOCH);

    enc.push(ciborium_ll::Header::Tag(1001))?;
    enc.push(ciborium_ll::Header::Map(Some(3)))?;

    // basetime
    if timestamp.as_secs() >= 0 {
        enc.push(BASE_TIME)?;
        enc.push(ciborium_ll::Header::Positive(timestamp.as_secs() as u64))?;
    } else {
        enc.push(BASE_TIME)?;
        enc.push(ciborium_ll::Header::Negative((!timestamp.as_secs()) as u64))?;
    }

    // elective nanoseconds
    if timestamp.subsec_nanos() >= 0 {
        enc.push(NANOSECONDS)?;
        enc.push(ciborium_ll::Header::Positive(
            timestamp.subsec_nanos() as u64
        ))?;
    } else {
        enc.push(NANOSECONDS)?;
        enc.push(ciborium_ll::Header::Positive(
            (1_000_000_000 + timestamp.subsec_nanos()) as u64,
        ))?;
    }

    // required timezone
    enc.push(TIME_ZONE)?;
    enc.text(zone, None)?;

    Ok(())
}

/// <https://www.rfc-editor.org/rfc/rfc9581.html>
fn zoned_cbor_9581_deser<E: std::fmt::Display>(
    dec: &mut Decoder<impl Read<Error = E>>,
) -> eyre::Result<Zoned> {
    // TODO: this is not very robust
    match dec.pull().map_err(map_deser_err)? {
        ciborium_ll::Header::Tag(1001) => {}
        header => bail!("invalid header: {header:?}"),
    };
    match dec.pull().map_err(map_deser_err)? {
        ciborium_ll::Header::Map(Some(3)) => {}
        header => bail!("invalid header: {header:?}"),
    };

    match dec.pull().map_err(map_deser_err)? {
        BASE_TIME => {}
        header => bail!("invalid header: {header:?}"),
    };
    let secs = match dec.pull().map_err(map_deser_err)? {
        ciborium_ll::Header::Positive(secs) => secs as i64,
        ciborium_ll::Header::Negative(secs) => !(secs as i64),
        header => bail!("invalid header: {header:?}"),
    };

    match dec.pull().map_err(map_deser_err)? {
        NANOSECONDS => {}
        header => bail!("invalid header: {header:?}"),
    };
    let nsecs = match dec.pull().map_err(map_deser_err)? {
        ciborium_ll::Header::Positive(secs) => secs as i32,
        ciborium_ll::Header::Negative(secs) => 1_000_000_000 + !(secs as i32),
        header => bail!("invalid header: {header:?}"),
    };

    let timestamp = Timestamp::new(secs, nsecs)?;

    match dec.pull().map_err(map_deser_err)? {
        TIME_ZONE => {}
        header => bail!("invalid header: {header:?}"),
    };
    let timezone = match dec.pull().map_err(map_deser_err)? {
        ciborium_ll::Header::Text(len) => {
            // we expect only a single segment here
            // and we expect timezone iana names to not be too long
            let mut scratch = [0; 256];

            let mut segments = dec.text(len);
            let Some(mut segment) = segments.pull().map_err(map_deser_err)? else {
                bail!("expecting 1 segment for child id")
            };
            let Some(tz) = segment.pull(&mut scratch[..]).map_err(map_deser_err)? else {
                bail!("expecting 1 segment for child id")
            };

            // check if this is an offset
            if tz.len() == 6 && tz.as_bytes()[3] == b':' {
                let hours = [tz.as_bytes()[1] - b'0', tz.as_bytes()[2] - b'0'];
                let minutes = [tz.as_bytes()[4] - b'0', tz.as_bytes()[5] - b'0'];

                let hours = (hours[0] * 10 + hours[1]) as i32;
                let minutes = (minutes[0] * 10 + minutes[1]) as i32;
                let mut seconds = hours * 3600 + minutes * 60;
                if tz.as_bytes()[0] == b'-' {
                    seconds = -seconds;
                }
                Offset::from_seconds(seconds)?.to_time_zone()
            } else {
                TimeZone::get(tz)?
            }
        }
        header => bail!("invalid header: {header:?}"),
    };

    Ok(Zoned::new(timestamp, timezone))
}

#[cfg(test)]
mod tests {
    use atuin_client::record::sqlite_store::SqliteStore;
    use atuin_common::record::{HostId, RecordId};
    use hex_literal::hex;
    use jiff::{civil::datetime, tz::TimeZone};
    use uuid::uuid;

    use crate::{Client, Note};

    const HOST1: HostId = HostId(uuid!("a64b4e78-435d-45e1-a7f2-8a9d34f6074a"));
    const HOST2: HostId = HostId(uuid!("f1ddfd0e-e3fd-47a7-9e6a-4998279546c9"));

    const FOO: RecordId = RecordId(uuid!("b7d8ac79-4e91-4af8-b164-6e14212531a8"));
    const BAR: RecordId = RecordId(uuid!("46924371-80e2-41ab-85ac-c44d9cb90d81"));
    const BAZ: RecordId = RecordId(uuid!("3965e843-d386-424d-9c32-f5d0d4234641"));

    #[tokio::test]
    async fn round_trip() {
        let store = SqliteStore::new(":memory:", 1.0).await.unwrap();

        let dt1 = datetime(2023, 12, 19, 11, 19, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap();
        let dt2 = datetime(2024, 11, 10, 12, 19, 22, 0)
            .to_zoned(TimeZone::get("Europe/Paris").unwrap())
            .unwrap();
        let dt3 = datetime(2024, 11, 9, 2, 0, 59, 0)
            .to_zoned(TimeZone::get("Europe/London").unwrap())
            .unwrap();

        let mut client1 = Client::test(store.clone(), HOST1);
        let mut client2 = Client::test(store.clone(), HOST2);

        let id1 = client1
            .add_record("Hello world".to_string(), vec![], dt1.clone())
            .await;
        let id2 = client1
            .add_record("Goodbye world".to_string(), vec![id1], dt2.clone())
            .await;
        let id3 = client2
            .add_record("Hello world again".to_string(), vec![], dt3.clone())
            .await;

        let mut loaded1 = vec![];
        client1
            .load_notes(|host, id, note| loaded1.push((host, id, note)))
            .await
            .unwrap();

        let mut loaded2 = vec![];
        client2
            .load_notes(|host, id, note| loaded2.push((host, id, note)))
            .await
            .unwrap();

        assert_eq!(
            loaded1,
            [(
                HOST2,
                id3,
                Note {
                    note: "Hello world again".to_string(),
                    children: vec![],
                    datetime: dt3,
                }
            )]
        );

        assert_eq!(
            loaded2,
            [
                (
                    HOST1,
                    id1,
                    Note {
                        note: "Hello world".to_string(),
                        children: vec![],
                        datetime: dt1,
                    }
                ),
                (
                    HOST1,
                    id2,
                    Note {
                        note: "Goodbye world".to_string(),
                        children: vec![id1],
                        datetime: dt2,
                    }
                )
            ]
        );
    }

    #[test]
    fn ser_deser1() {
        let note = Note {
            note: "This is my note".to_owned(),
            datetime: datetime(2024, 11, 9, 12, 19, 22, 0)
                .to_zoned(TimeZone::get("Europe/Paris").unwrap())
                .unwrap(),
            children: vec![FOO, BAR, BAZ],
        };

        let buf = note.ser_v0_to_vec();
        let note2 = Note::deser_v0(&buf).unwrap();
        assert_eq!(note, note2);

        assert_eq!(
            buf,
            hex!(
                "83" // array(3)
                    // note
                    "6f" // text(15)
                        "54686973206973206d79206e6f7465"

                    // datetime
                    "d9 03e9" // tag(extended_time)
                        "a3" // map(3)
                           "01 1a 672f453a" // basetime: unsigned(1,731,151,162)
                           "28 00" // nanoseconds: 0
                           "0a 6c" // timezone: text(12)
                              "4575726f70652f5061726973" // "Europe/Paris"

                    // children
                    "83" // array(3)
                        "d8 25" // tag(uuid_bytes)
                            "50" // bytes(16)
                                "b7d8ac794e914af8b1646e14212531a8"
                        "d8 25" // tag(uuid_bytes)
                            "50" // bytes(16)
                                "4692437180e241ab85acc44d9cb90d81"
                        "d8 25" // tag(uuid_bytes)
                            "50" // bytes(16)
                                "3965e843d386424d9c32f5d0d4234641"
            )
        );
    }

    #[test]
    fn ser_deser2() {
        const FOO: RecordId = RecordId(uuid!("b7d8ac79-4e91-4af8-b164-6e14212531a8"));
        const BAR: RecordId = RecordId(uuid!("46924371-80e2-41ab-85ac-c44d9cb90d81"));
        const BAZ: RecordId = RecordId(uuid!("3965e843-d386-424d-9c32-f5d0d4234641"));

        let note = Note {
            note: "This is my note".to_owned(),
            datetime: datetime(2024, 11, 9, 6, 19, 22, 0)
                .to_zoned(TimeZone::get("America/New_York").unwrap())
                .unwrap(),
            children: vec![FOO, BAR, BAZ],
        };

        let buf = note.ser_v0_to_vec();
        let note2 = Note::deser_v0(&buf).unwrap();
        assert_eq!(note, note2);

        assert_eq!(
            buf,
            hex!(
                "83" // array(3)
                    // note
                    "6f" // text(15)
                        "54686973206973206d79206e6f7465"

                    // datetime
                    "d9 03e9" // tag(extended_time)
                        "a3" // map(3)
                           "01 1a 672f453a" // basetime: unsigned(1,731,151,162)
                           "28 00" // nanoseconds: 0
                           "0a 70" // timezone: text(16)
                              "416d65726963612f4e65775f596f726b" // "America/New_York"

                    // children
                    "83" // array(3)
                        "d8 25" // tag(uuid_bytes)
                            "50" // bytes(16)
                                "b7d8ac794e914af8b1646e14212531a8"
                        "d8 25" // tag(uuid_bytes)
                            "50" // bytes(16)
                                "4692437180e241ab85acc44d9cb90d81"
                        "d8 25" // tag(uuid_bytes)
                            "50" // bytes(16)
                                "3965e843d386424d9c32f5d0d4234641"
            )
        );
    }
}
