import { LaunchProps, getPreferenceValues } from "@raycast/api";
import { execFile } from "child_process";
import { promisify } from "util";

interface QuickNoteDraft {
}

interface QuickNoteArgs {
  note: string,
}

interface Preferences {
  cmdPath: string,
}

export default async function QuickNote(props: LaunchProps<{ draftValues: QuickNoteDraft, arguments: QuickNoteArgs }>) {
  const { cmdPath } = getPreferenceValues<Preferences>();
  const { arguments: { note } } = props;

  const { stdout, stderr } = await promisify(execFile)(cmdPath, ["record", note]);
  console.log({ stdout, stderr });
}
