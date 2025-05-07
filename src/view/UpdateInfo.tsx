import { check, type Update } from "@tauri-apps/plugin-updater";
import { useEffect, useState } from "react";
import { Updater } from "./Updater";

type UpdateInfoUnfetched = {
  type: "unfetched";
};

type UpdateInfoFetched = {
  type: "fetched";
  update: Update;
};

type UpdateInfoError = {
  type: "error";
  error: string;
};

type UpdateInfoNoUpdate = {
  type: "no-update";
};

type UpdateInfoStauts =
  | UpdateInfoUnfetched
  | UpdateInfoFetched
  | UpdateInfoError
  | UpdateInfoNoUpdate;

export const UpdateInfo = () => {
  const [updateInfo, setUpdateInfo] = useState<UpdateInfoStauts>({
    type: "unfetched",
  });
  useEffect(() => {
    check()
      .then((update) => {
        if (update) {
          setUpdateInfo({
            type: "fetched",
            update,
          });
        } else {
          setUpdateInfo({
            type: "no-update",
          });
        }
      })
      .catch((errorMsg) => {
        setUpdateInfo({
          type: "error",
          error: errorMsg,
        });
      });
  }, []);
  const createInstallProcess = (update: Update) => {
    return () => {
      update
        .downloadAndInstall()
        .then(() => {
          console.log("Update downloaded and installed");
        })
        .catch((errorMsg) => {
          console.error("Error downloading and installing update:", errorMsg);
        });
    };
  };
  switch (updateInfo.type) {
    case "unfetched":
      return <div>Checking for updates...</div>;
    case "fetched":
      return <Updater update={updateInfo.update} />;
    case "no-update": {
      return (
        <div>
          <h2>Update Info</h2>
          <p>No updates available</p>
        </div>
      );
    }
    case "error": {
      return (
        <div>
          <h2>Update Info</h2>
          <p>Error checking for updates: {updateInfo.error}</p>
        </div>
      );
    }
  }
};
