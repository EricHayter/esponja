<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let listPromise = getList()
    function getList() {
        return invoke('some_list' , {})
    }
</script>
    
<div>
    {#await listPromise then list}
    <p>{list}</p>
    {/await} 
</div>
