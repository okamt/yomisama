<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from "@tauri-apps/api/window";
  import type { ComponentType, SvelteComponent } from "svelte";
  import type { Payload } from "./bindings";
  import FirstTimeSetup from "./lib/FirstTimeSetup.svelte";
  import Loading from "./lib/Loading.svelte";
  import Query from "./lib/Query.svelte";

  const components = { FirstTimeSetup, Query };

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

  const getComponent = (): ComponentType<SvelteComponent> =>
    components[payload!.component];
  const getProps = () => (({ component, ...props }) => props)(payload!);
</script>

{#await init}
  <Loading />
{:then _}
  <main class="container">
    <svelte:component this={getComponent()} {...getProps()} />
  </main>
{/await}
