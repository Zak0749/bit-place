<script lang="ts">
import { subscribe } from 'svelte/internal';

  import socket from '../helpers/socket'
  import popup from '../helpers/popup'

  let point = {
    x: 0,
    y: 0,
    color: '#000000'
  }

  const onSubmit = () => {
    socket.send({ x: point.y / 5, y: point.x / 5, color: point.color.replace("#", "") })
  }

  popup.subscribe(data => {
    if (!data) return
    point.x = data.top
    point.y = data.left
  })
</script>

<input on:change={onSubmit} type="color"  bind:value={point.color} style={`top:${point.x}px; left:${point.y}px;`}>

<style>
  input {
    width: 30px;
    height: 30px;
    margin: 0;
    position: absolute;

    background-color: rgba(195, 195, 195, 0.5);
    padding: 0;
    border: none;
  }
</style>