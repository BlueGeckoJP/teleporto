<script lang="ts">
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import FileInput from "./lib/FileInput.svelte";
  import Input from "./lib/Input.svelte";
  import Button from "./lib/Button.svelte";
  import { open, type OpenDialogOptions } from "@tauri-apps/api/dialog";

  export let filePath = "";
  export let ipAddress = "";

  export let receiveMessage = "Waiting to receive file...";
  export let fromAddressMessage = "-.-.-.-";

  appWindow.setSize(new LogicalSize(500, 400));

  window.__TAURI__.invoke("init_web_server");

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

  async function acceptButton() {
    window.__TAURI__.event.emit("front-to-back", "FRONT-TO-BACK");
  }
</script>

<main>
  <div>
    <div id="send-container">
      <table id="send-table">
        <tr>
          <td>File Path:</td>
          <td>
            <div>
              <FileInput bind:value={filePath} func={openFileDialog} />
            </div>
          </td>
        </tr>
        <tr>
          <td>IP Address:</td>
          <td><Input bind:value={ipAddress} /></td>
        </tr>
      </table>
      <Button text="Send" func={() => {}} />
    </div>
    <div id="receive-container">
      <p>{receiveMessage}</p>
      <p>From: {fromAddressMessage}</p>
      <Button text="Accept" func={acceptButton} />
    </div>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
  }

  /** {
    border: 1px dotted red;
  }*/

  main > div {
    box-sizing: border-box;
    width: 90%;
    height: 100vh;
    margin: 0 auto;
    border-left: 1px solid black;
    border-right: 1px solid black;
  }

  #send-container {
    box-sizing: border-box;
    width: 100%;
    height: 50vh;
    border-bottom: 1px solid black;
  }

  #send-table {
    width: 100%;
  }

  #receive-container {
    width: 100%;
    height: calc(50vh - 16px);
    text-align: center;
  }
</style>
