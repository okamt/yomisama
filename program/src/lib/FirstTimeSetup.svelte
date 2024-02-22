<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from "@tauri-apps/api/window";

  const STEPS = 3;
  let step = 0;
  export let defaultConfigDir = "";
  let errorMessage = "";
  let dictModal: HTMLDialogElement;

  async function chooseConfigDir(path: any): Promise<any> {
    try {
      await invoke("set_config_dir", { path });
      return (step = 1);
    } catch (error: any) {
      return (errorMessage = error.toString());
    }
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
                      >{defaultConfigDir}</span
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
        <div class="flex justify-center gap-x-8">
          <div>
            <input
              type="file"
              class="file-input file-input-bordered w-full max-w-xs"
            />
          </div>
          <div class="self-center">
            <button class="btn" on:click={() => (step = 2)}> OK </button>
          </div>
        </div>
        <button class="btn" on:click={() => dictModal.showModal()}
          >open modal</button
        >
        <dialog bind:this={dictModal} class="modal">
          <div class="modal-box">
            <!--<form method="dialog">
              <button
                class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                >✕</button
              >
            </form>-->
            <!--<h3 class="font-bold text-lg">Hello!</h3>-->
            <!--<p class="py-4"></p>-->
            <div
              class="overflow-x-auto flex flex-col justify-items-center space-y-8"
            >
              <table class="table">
                <tbody>
                  <tr>
                    <th>Title</th>
                    <td>JMDict</td>
                  </tr>
                  <tr>
                    <th>Authors</th>
                    <td
                      >Electronic Dictionary Research and Development Group
                      (http://www.edrdg.org/edrdg/licence.html)</td
                    >
                  </tr>
                  <tr>
                    <th>Version</th>
                    <td>1.09</td>
                  </tr>
                  <tr>
                    <th>Homepage</th>
                    <td>https://github.com/scriptin/jmdict-simplified</td>
                  </tr>
                </tbody>
              </table>
              <div class="flex gap-8 items-center self-center">
                <button class="btn btn-primary outline-none">Import</button>
                <form method="dialog">
                  <button class="btn outline-none">Cancel</button>
                </form>
              </div>
            </div>
          </div>
        </dialog>
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
