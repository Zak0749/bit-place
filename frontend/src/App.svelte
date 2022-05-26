<script lang="ts">
  import Grid from './components/grid.svelte'
  import Editor from './components/editor.svelte'
</script>

<main >
  <Grid><Editor /></Grid>
    
</main>


<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  main {
    position: relative;
    width: 500px;
    height: 500px;
  }
</style>