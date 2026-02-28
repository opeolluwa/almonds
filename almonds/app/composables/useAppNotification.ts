export type NotificationType = "info" | "success" | "warning" | "error";

export interface AppNotification {
  message: string;
  type?: NotificationType;
  duration?: number; // ms — 0 = persistent
}

export function useAppNotification() {
  const notification = useState<AppNotification | null>(
    "appNotification",
    () => null,
  );

  let timer: ReturnType<typeof setTimeout> | null = null;

  function notify(config: AppNotification) {
    if (timer) clearTimeout(timer);

    notification.value = { type: "info", duration: 3500, ...config };

    const duration = notification.value.duration!;
    if (duration > 0) {
      timer = setTimeout(() => dismiss(), duration);
    }
  }

  function dismiss() {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
    notification.value = null;
  }

  return {
    notification: readonly(notification),
    notify,
    dismiss,
  };
}
