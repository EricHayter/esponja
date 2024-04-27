<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let phrasesPromise = getPhrases()
    function getPhrases() {
        return invoke('get_phrases' , {})
    }
</script>
    
<div>
    {#await phrasesPromise}
    <p>Getting Phrases...</p> 

    {:then phrases}

    <div class="card" style="width: 18rem;">
      <ul class="list-group list-group-flush">

        {#each phrases as phrase}
        <li class="list-group-item">{phrase[0]}</li>
        {/each}
      </ul>
    </div>

    {:catch error}
    <p>Something broke</p>
    <p>{error}</p>
    {/await} 
</div>
