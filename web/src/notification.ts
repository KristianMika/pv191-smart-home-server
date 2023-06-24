const VOC_INDEX_NOTIFICATION_THRESHOLD = 150;
const TEN_MINUTES = 1000 * 60 * 10;

export const shouldGenerateVocAlert = (vocIndex?: number): boolean => {
  const lastNotificationTime = parseInt(
    localStorage.getItem("lastNotificationTime") || ""
  );
  const decision =
    vocIndex &&
    vocIndex > VOC_INDEX_NOTIFICATION_THRESHOLD &&
    (!lastNotificationTime || lastNotificationTime < Date.now() - TEN_MINUTES);
  return decision as boolean;
};

export const generateHIghVocNotification = (vocIndex: number) => {
  new Notification("High VOC alert", {
    body: `We have detected VOC of ${vocIndex}!`,
    icon: "voc-notification-icon.png",
  });
};

export const checkVocLevel = (vocIndex?: number) => {
  if (shouldGenerateVocAlert(vocIndex)) {
    localStorage.setItem("lastNotificationTime", Date.now().toString());
    generateHIghVocNotification(vocIndex as number);
  }
};
