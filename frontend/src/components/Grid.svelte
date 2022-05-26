<script lang="ts">
  import { onMount } from "svelte";
  import type popup from '../helpers/popup';
  import socket from '../helpers/socket';
  import popup from '../helpers/popup';

  let canvas;
  let ctx;

  onMount(async () => {
    ctx = canvas.getContext("2d");

    let grid = await (await fetch(`/api/grid`)).json();

    grid.forEach(element => {
			ctx.fillStyle = `#${element.color}`;
      ctx.fillRect(element.x * 5, element.y * 5, 5, 5)
    });

    socket.subscribe(data => {
			ctx.fillStyle = `#${data.color}`;
      ctx.fillRect(data.x * 5, data.y * 5, 5, 5)
		})
  })

  const click = () => {
    const rect = canvas.getBoundingClientRect()
    const x = Math.round((event.clientX - rect.left) / 5) * 5
    const y = Math.round((event.clientY - rect.top) / 5) * 5
    popup.set(y,x)
  }
  
</script>

<canvas bind:this={canvas} width={500} height={500} on:click={click}></canvas>
<slot></slot>
