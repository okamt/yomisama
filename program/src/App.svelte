<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from "@tauri-apps/api/window";
  import type { Payload } from "./bindings";
  import FirstTimeSetup from "./lib/FirstTimeSetup.svelte";
  import Loading from "./lib/Loading.svelte";

  let payload: Payload | undefined = undefined;

  appWindow.onCloseRequested(async (_event) => {
    await invoke("window_unloading");
  });

  const init = (async function () {
    if (payload !== undefined) {
      return;
    }

    payload = await invoke("window_loaded");
  })();
</script>

{#await init}
  <Loading />
{:then _}
  <main class="container">
    <FirstTimeSetup {payload} />
  </main>
{/await}
