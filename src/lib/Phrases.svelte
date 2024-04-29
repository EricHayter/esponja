<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let phrasesPromise = getPhrases()
    function getPhrases() {
        return invoke('get_phrases' , {})
    }
</script>

<div>
    {#await phrasesPromise}
    
    {:then phrases}

    <ul>
        {#each phrases as phrase}
        <li>{phrase[0]}</li>
        {/each}
    </ul>

    {:catch error}
    <p>Something broke</p>
    <p>{error}</p>
    {/await} 
</div>
