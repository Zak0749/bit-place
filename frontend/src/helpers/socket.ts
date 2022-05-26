import { writable, type Writable } from "svelte/store";

type Point = {
  x: number;
  y: number;
  color: string;
};

const messageStore: Writable<Point> = writable(null);

/** @type {WebSocket | null} */
const socket = new WebSocket(`ws://${location.host}/ws`);

socket.onmessage = (point) => {
  messageStore.set(JSON.parse(point.data));
};

const send = (point) => {
  console.log(point);
  socket.send(JSON.stringify(point));
};

export default {
  subscribe: messageStore.subscribe,
  send,
};
