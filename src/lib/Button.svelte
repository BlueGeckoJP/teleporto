<script>
  export let text = "";
  export let func = () => {};
</script>

<main>
  <div id="top">
    <button id="button" on:click={func}>{text}</button>
  </div>
</main>

<style>
  * {
    box-sizing: border-box;
  }

  #top {
    margin-top: 16px;
    display: flex;
  }

  #button {
    flex-grow: 1;
    border: none;
    outline: none;
    border-radius: 0%;
    background: none;
    box-shadow: none;
    border-top: 1px solid gray;
    border-bottom: 1px solid gray;
  }

  #button:active {
    transition: all 0.5s;
    border-top: 1px solid aqua;
    border-bottom: 1px solid aqua;
  }
</style>
