<script lang="ts">
    export let offerCount: number;
    export let requestCount: number;

    $: totalCount = offerCount + requestCount;
</script>

<div
    class="w-8 h-8 rounded-full border-2 border-base-500 flex items-center justify-center font-black textarea-neutral text-lg shadow-lg bg-base-100"
    title="{offerCount} offers, {requestCount} requests"
>
    {totalCount}
</div>
