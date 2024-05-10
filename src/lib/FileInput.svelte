<script>
  export let value = "";
  export let func = () => {};
</script>

<main>
  <div id="top">
    <div>
      <input bind:value id="input" />
      <div id="input-underline"></div>
    </div>
    <button id="button" on:click={func}>..</button>
  </div>
</main>

<style>
  * {
    box-sizing: border-box;
  }

  #top {
    display: flex;
  }

  #top > div {
    flex-grow: 1;
  }

  #input {
    font-size: 16px;
    width: 100%;
    outline: none;
    border: none;
    border-radius: 0%;
    background: none;
    box-shadow: none;
    padding-bottom: 8px;
    box-sizing: border-box;
  }

  #input-underline {
    position: relative;
    border-top: 1px solid gray;
  }

  #input-underline::before,
  #input-underline::after {
    position: absolute;
    bottom: 0px;
    width: 0px;
    height: 1px;
    content: "";
    background-color: aqua;
    transition: all 0.5s;
  }

  #input-underline::before {
    left: 50%;
  }

  #input-underline::after {
    right: 50%;
  }

  #input:focus + #input-underline::before,
  #input:focus + #input-underline::after {
    width: 50%;
  }

  #button {
    margin-left: 4px;
    border: none;
    outline: none;
    border-radius: 0%;
    background: none;
    box-shadow: none;
  }
</style>
