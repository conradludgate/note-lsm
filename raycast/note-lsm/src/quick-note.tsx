import { Form, ActionPanel, Action, popToRoot, LaunchProps, Detail, getPreferenceValues } from "@raycast/api";

interface QuickNoteDraft {
  tags: string[],
}

interface QuickNoteArgs {
  note: string,
}

interface Preferences {
  tags: string[] | undefined,
}

export default function QuickNote(props: LaunchProps<{ draftValues: QuickNoteDraft, arguments: QuickNoteArgs }>) {
  const { draftValues, arguments: { note } } = props;
  const { tags } = getPreferenceValues<Preferences>();

  console.log(note);

  return (
    <Form
      enableDrafts
      actions={
        <ActionPanel>
          <Action.SubmitForm
            onSubmit={(values: QuickNoteDraft) => {
              console.log("onSubmit", { note, tags: values.tags });
              popToRoot();
            }}
          />
        </ActionPanel>
      }
    >
      <Detail markdown={note} />
      <Form.TagPicker id="tags" title="Tags" defaultValue={draftValues?.tags}>
        {tags?.map((tag, i) => <Form.TagPicker.Item key={i} value={tag} title={tag} />)}
      </Form.TagPicker>
    </Form>
  );
}
