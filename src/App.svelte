<script lang="ts">
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import { open, type OpenDialogOptions } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";

  export let filePath = "";
  export let ipAddress = "";

  export let receiveMessage = "Waiting to receive file...";
  export let fromAddressMessage = "-.-.-.-";

  appWindow.setSize(new LogicalSize(500, 400));
  invoke("init_web_server");

  async function openFileDialog() {
    let options: OpenDialogOptions = {
      directory: false,
      multiple: false,
    };
    let result = await open(options);
    if (!Array.isArray(result) && result !== null) {
      filePath = result;
    }
  }

  async function sendButton() {
    await invoke("send_file", { path: filePath, dst_ip: ipAddress });
  }

  async function acceptButton() {
    window.__TAURI__.event.emit("front-to-back", "FRONT-TO-BACK");
    receiveMessage = "Waiting to receive file...";
    fromAddressMessage = "-.-.-.-";
  }

  async function listener() {
    await window.__TAURI__.event.listen("back-to-front", (event) => {
      console.log("back-to-front");
      receiveMessage = "Received file available";
      fromAddressMessage = event.payload as string;
    });
  }

  listener();
</script>

<main>
  <div id="top">
    <div id="send-container">
      <div class="heading-text">
        <p>Send</p>
        <div></div>
      </div>
      <div id="send-config-container">
        <p>File Path:</p>
        <div id="filepath-input">
          <input class="input-theme" bind:value={filePath} />
          <button class="button-theme" on:click={openFileDialog}>..</button>
        </div>
        <p>IP Address:</p>
        <input
          id="ipaddress-input"
          class="input-theme"
          bind:value={ipAddress}
        />
      </div>
      <div class="flex-grow-button">
        <button class="button-theme" on:click={sendButton}>Send</button>
      </div>
    </div>
    <div id="recv-container">
      <div class="heading-text">
        <p>Receive</p>
        <div></div>
      </div>
      <p class="recv-msg">{receiveMessage}</p>
      <p class="recv-msg">From: {fromAddressMessage}</p>
      <div class="flex-grow-button">
        <button class="button-theme" on:click={acceptButton}>Accept</button>
      </div>
    </div>
  </div>
</main>

<style>
  :root {
    --first-color: #f9f7f7;
    --second-color: #dbe2ef;
    --third-color: #3f72af;
    --primary-text-color: #212121;
  }

  :global(body) {
    margin: 0;
  }

  main {
    width: 100%;
    height: 100vh;
    background-color: var(--first-color);
  }

  p {
    margin: 0;
    color: var(--primary-text-color);
  }

  #top {
    width: 100%;
    height: 100vh;
    padding-left: 20px;
    padding-right: 20px;
  }

  #send-container {
    width: 100%;
    height: calc(50vh - 20px);
    padding-top: 20px;
  }

  #recv-container {
    width: 100%;
    height: calc(50vh - 20px);
    padding-bottom: 20px;
  }

  #filepath-input {
    display: flex;
    width: calc(100% - 40px);
  }

  #filepath-input > input {
    width: 100%;
  }

  #filepath-input > button {
    width: 30px;
  }

  #ipaddress-input {
    width: calc(100% - 40px);
  }

  .flex-grow-button {
    margin-top: 8px;
    display: flex;
    width: calc(100% - 40px);
  }

  .flex-grow-button > button {
    flex-grow: 1;
  }

  .recv-msg {
    width: calc(100% - 40px);
    display: block;
    text-align: center;
  }

  .input-theme {
    border: 2px solid var(--second-color);
    border-radius: 5px;
    background-color: var(--first-color);
    transition: all 0.3s;
    color: var(--primary-text-color);
  }

  .input-theme:focus {
    border: 2px solid var(--third-color);
    outline: 0;
  }

  .button-theme {
    border: 0;
    border-radius: 5px;
    background-color: var(--second-color);
    transition: all 0.5s;
    color: var(--primary-text-color);
  }

  .button-theme:active {
    background-color: var(--third-color);
  }

  .heading-text {
    width: calc(100% - 40px);
    display: flex;
    align-items: center;
  }

  .heading-text > p {
    font-weight: bold;
  }

  .heading-text > div {
    border: 1px solid var(--primary-text-color);
    width: 100%;
    height: 0;
    margin-left: 4px;
  }
</style>
