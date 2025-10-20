<script lang="ts">
	let {
		size = 24,
		class: className = '',
		iconName
	}: {
		size?: number;
		class?: string;
		iconName: string;
	} = $props();

	let iconUrl = $derived(`https://cdn.simpleicons.org/${iconName}`);

	const fallbackIcon =
		'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iMTAiIHN0cm9rZT0iY3VycmVudENvbG9yIiBzdHJva2Utd2lkdGg9IjIiLz4KPHA=';

	let imgElement: HTMLImageElement | undefined = $state();

	function handleError() {
		if (imgElement) {
			imgElement.src = fallbackIcon;
		}
	}
</script>

<img
	bind:this={imgElement}
	src={iconUrl}
	alt="{iconName} icon"
	width={size}
	height={size}
	class={className}
	onerror={handleError}
/>
