import { LaunchProps, getPreferenceValues, showHUD } from "@raycast/api";
import { execFile } from "child_process";
import { promisify } from "util";

interface ReminderDraft {
}

interface ReminderArgs {
}

interface Preferences {
  cmdPath: string,
}

export default async function Reminder(props: LaunchProps<{ draftValues: ReminderDraft, arguments: ReminderArgs }>) {
  await showHUD("Don't forget to take and review your notes");
}
