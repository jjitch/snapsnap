import { check, type Update } from "@tauri-apps/plugin-updater";
import { useEffect, useState } from "react";

export const UpdateInfo = () => {
  const [update, setUpdate] = useState<Update | null>(null);
  const [updateError, setUpdateError] = useState<string>("");
  useEffect(() => {
    check()
      .then((update) => {
        if (update) {
          setUpdate(update);
        }
      })
      .catch((errorMsg) => {
        setUpdateError(errorMsg);
      });
  }, []);
  return (
    <div>
      <h2>Update Info</h2>
      <p>
        {update
          ? `found update ${update.version} from ${update.date} with notes ${update.body}. Current version is ${update.currentVersion}`
          : `no update found: ${updateError}`}
      </p>
    </div>
  );
};
