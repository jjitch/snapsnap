import type { Update } from "@tauri-apps/plugin-updater";
import { useState } from "react";

export const Updater = (props: { update: Update }) => {
  const { update } = props;
  const [isFinished, setIsFinished] = useState(false);
  return (
    <div>
      <h2>Update Info</h2>
      <p>
        found update {update.version} from {update.date} with notes
        {update.body}. Current version is {update.currentVersion}
      </p>
      <input
        type="button"
        onClick={() =>
          update.downloadAndInstall().then(() => {
            setIsFinished(true);
          })
        }
        value="Download and Install Update"
      />
      {isFinished && <p>Update downloaded and installed</p>}
    </div>
  );
};
