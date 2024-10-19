import { Form, ActionPanel, Action, popToRoot, LaunchProps, getPreferenceValues } from "@raycast/api";
import { useExec, useFrecencySorting } from "@raycast/utils";
import { execFile } from "child_process";
import { useMemo } from "react";
import { promisify } from "util";

interface QuickNoteDraft {
  tags: string[],
}

interface QuickNoteArgs {
  note: string,
}

interface Preferences {
  cmdPath: string,
}

interface Tag {
  tag: string,
  description: string | null,
}

export default function QuickNote(props: LaunchProps<{ draftValues: QuickNoteDraft, arguments: QuickNoteArgs }>) {
  const { cmdPath } = getPreferenceValues<Preferences>();
  const { isLoading, data } = useExec(cmdPath, ["--output=json", "tags", "list"]);

  const tags = useMemo<Tag[]>(() => {
    const tags: Tag[] = JSON.parse(data || "[]");
    // return tags.map(tag => ({ id: tag.tag, description: tag.description }));
    return tags
  }, [data]);
  const { data: sortedTags, visitItem } = useFrecencySorting<Tag>(tags, {
    key: (tag) => tag.tag
  });

  const { draftValues, arguments: { note } } = props;

  return (
    <Form
      isLoading={isLoading}
      enableDrafts
      actions={
        <ActionPanel>
          <Action.SubmitForm
            onSubmit={async (values: QuickNoteDraft) => {
              await Promise.all(values.tags.map(tag => visitItem(JSON.parse(tag))));

              const tags = values.tags.flatMap((tag) => {
                const t: Tag = JSON.parse(tag);
                return ["--tag", t.tag]
              });

              const { stdout, stderr } = await promisify(execFile)(cmdPath, ["record", ...tags, note]);
              console.log({ stdout, stderr });

              await popToRoot();
            }}
          />
        </ActionPanel>
      }
    >
      <Form.TagPicker id="tags" title="Tags" defaultValue={draftValues?.tags}>
        {sortedTags?.map((tag) => <Form.TagPicker.Item
          key={tag.tag}
          value={JSON.stringify(tag)}
          title={tag.description ?? tag.tag}
        />)}
      </Form.TagPicker>
    </Form>
  );
}
