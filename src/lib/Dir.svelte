<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let dirNamePromise = getDirName()
    function getDirName() {
        return invoke('some_command' , {})
    }
</script>
    
<div>
    {#await dirNamePromise}
    <p>Waiting...</p> 
    {:then dirName}
    <p>dir name</p>
    <p>{dirName}</p>
    {:catch error}
    <p>Something broke</p>
    <p>{error}</p>
    {/await} 
</div>
