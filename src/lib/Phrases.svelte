<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let phrasesPromise = getPhrases()
    function getPhrases() {
        return invoke('get_phrases' , {})
    }
</script>

<div>
    {#await phrasesPromise}
    <div class="preloader-wrapper big active">
        <div class="spinner-layer spinner-blue">
            <div class="circle-clipper left">
                <div class="circle"></div>
            </div><div class="gap-patch">
                <div class="circle"></div>
            </div><div class="circle-clipper right">
                <div class="circle"></div>
            </div>
        </div>
    </div>

    {:then phrases}

    <ul class="collection gap-5">
        {#each phrases as phrase}
        <li>{phrase[0]}</li>
        {/each}
    </ul>

    {:catch error}
    <p>Something broke</p>
    <p>{error}</p>
    {/await} 
</div>
