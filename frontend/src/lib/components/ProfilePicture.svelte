<script lang="ts">
    import { SquirrelSvg } from './icons';
    
    export let profilePicture: string | null = null;
    export let name: string = '';
    export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
    export let editable: boolean = false;
    export let onImageChange: ((file: File) => void) | null = null;
    
    let fileInput: HTMLInputElement;
    
    const sizeClasses = {
        sm: 'w-8 h-8',
        md: 'w-12 h-12', 
        lg: 'w-24 h-24',
        xl: 'w-32 h-32'
    };
    
    const textSizes = {
        sm: 'text-xs',
        md: 'text-sm',
        lg: 'text-2xl',
        xl: 'text-3xl'
    };
    
    function handleImageClick() {
        if (editable && fileInput) {
            fileInput.click();
        }
    }
    
    function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        const file = target.files?.[0];
        
        if (file && onImageChange) {
            if (!file.type.startsWith('image/')) {
                alert('Please select an image file');
                return;
            }
            
            if (file.size > 5 * 1024 * 1024) { // 5MB limit
                alert('Image must be less than 5MB');
                return;
            }
            
            onImageChange(file);
        }
    }
    
    function getInitials(name: string): string {
        return name
            .split(' ')
            .map(n => n.charAt(0))
            .join('')
            .toUpperCase()
            .slice(0, 2);
    }

    $: tooltipText = profilePicture ? 'Change picture' : 'Upload picture';
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div 
    class="avatar group {editable ? 'cursor-pointer tooltip tooltip-right' : ''}" 
    data-tip={editable ? tooltipText : null}
    on:click={handleImageClick} 
    on:keydown={(e) => e.key === 'Enter' && handleImageClick()} 
    role={editable ? 'button' : 'img'} 
    tabindex={editable ? 0 : -1}
>
    <div class="rounded-full overflow-hidden {sizeClasses[size]} {editable ? 'group-hover:blur-[1px] transition-all' : ''}">
        {#if profilePicture}
            <img 
                src={profilePicture} 
                alt="{name}'s profile" 
                class="rounded-full object-cover w-full h-full"
                loading="lazy"
            />
        {:else if name}
            <div class="bg-primary text-primary-content rounded-full w-full h-full flex items-center justify-center font-bold {textSizes[size]}">
                {getInitials(name)}
            </div>
        {:else}
            <div class="bg-base-300 text-base-content rounded-full w-full h-full flex items-center justify-center p-2">
                <SquirrelSvg />
            </div>
        {/if}
    </div>
    
    {#if editable}
        <input
            bind:this={fileInput}
            type="file"
            accept="image/*"
            class="hidden"
            on:change={handleFileChange}
        />
    {/if}
</div>