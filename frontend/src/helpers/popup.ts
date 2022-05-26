import { writable, type Writable } from "svelte/store";

const messageStore: Writable<{ top: number; left: number }> = writable(null);

export default {
  subscribe: messageStore.subscribe,
  set: (top, left) => {
    messageStore.set({ top, left });
  },
};
