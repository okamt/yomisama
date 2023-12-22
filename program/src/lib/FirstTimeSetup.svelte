<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from "@tauri-apps/api/window";

  const STEPS = 3;
  let step = 0;
  export let payload: any;
  let errorMessage = "";

  function chooseConfigDir(path: any): Promise<any> {
    return invoke("set_config_dir", { path })
      .then(() => (step = 1))
      .catch((error) => (errorMessage = error));
  }

  async function chooseAnotherConfigDir() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select configuration directory",
    });

    if (selected !== null) {
      await chooseConfigDir(selected);
    }
  }

  async function closeWindow() {
    await appWindow.close();
  }

  $: step, (errorMessage = "");
</script>

<div class="flex flex-col h-screen justify-between">
  {#key step}
    <main class="p-8 flex flex-col space-y-8">
      {#if step == 0}
        <h1 class="text-center text-3xl">First time setup</h1>
        <div class="flex flex-col items-center gap-4">
          <div>
            <div class="flex flex-col w-full">
              <button on:click={() => chooseConfigDir(null)} class="btn">
                <div>
                  <div>Use default configuration directory</div>
                  <div class="text-xs pt-1">
                    <span class="break-all font-mono font-normal"
                      >{payload.defaultConfigDir}</span
                    >
                  </div>
                </div>
              </button>
              <div class="divider">or</div>
              <button on:click={() => chooseAnotherConfigDir()} class="btn">
                Choose another directory
              </button>
            </div>
          </div>
          <div class="text-xs pr-16 pl-16 text-center">
            The configuration directory is where your settings, dictionaries and
            statistics will be stored.
          </div>
        </div>
      {:else if step == 1}
        <h1 class="text-center text-3xl">Import dictionaries</h1>
        <div class="self-center">
          <button class="btn" on:click={() => (step = 2)}> TODO </button>
        </div>
      {:else if step == 2}
        <h1 class="text-center text-3xl">All done!</h1>
        <div class="self-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="w-32 h-32"
          >
            <path
              fill-rule="evenodd"
              d="M19.916 4.626a.75.75 0 01.208 1.04l-9 13.5a.75.75 0 01-1.154.114l-6-6a.75.75 0 011.06-1.06l5.353 5.353 8.493-12.739a.75.75 0 011.04-.208z"
              clip-rule="evenodd"
            />
          </svg>
        </div>
        <p class="text-center">
          Yomisama is now ready to use. Settings can be changed at any time
          through the tray icon.
        </p>
        <div class="self-center">
          <button class="btn" on:click={closeWindow}>OK</button>
        </div>
      {/if}
      {#if errorMessage}
        <div class="text-xs pr-8 pl-8 text-center text-red-500">
          {errorMessage}
        </div>
      {/if}
    </main>
  {/key}
  <footer class="footer footer-center pb-8">
    <aside>
      <ul class="steps">
        {#each { length: step + 1 } as _}
          <li data-content="" class="step step-primary"></li>
        {/each}
        {#each { length: STEPS - (step + 1) } as _}
          <li data-content="" class="step"></li>
        {/each}
      </ul>
    </aside>
  </footer>
</div>
