<script lang="ts">
  interface RichOption {
    id: string;
    title: string;
    description: string;
    value?: any;
    disabled?: boolean;
    category?: string;
    metadata?: any;
  }

  export let mode: 'radio' | 'checkbox' = 'checkbox';
  export let name: string = '';
  export let options: RichOption[] = [];
  export let selectedValue: string | null = null; // For radio mode
  export let selectedValues: string[] = []; // For checkbox mode
  export let onChange: (value: string | string[]) => void;
  export let columns: number = 1; // Grid columns
  
  // Helper functions for styling
  function getCardClasses(option: RichOption, isSelected: boolean) {
    let baseClasses = "flex items-start space-x-3 cursor-pointer p-3 border rounded-lg transition-colors";
    
    if (option.disabled) {
      return baseClasses + " opacity-50 cursor-not-allowed bg-gray-800/30 border-gray-600";
    } else if (isSelected) {
      return baseClasses + " bg-gray-700/30 border-gray-500 hover:bg-gray-700/40";
    } else {
      return baseClasses + " bg-gray-700/20 border-gray-600 hover:bg-gray-700/30";
    }
  }
  
  function getTitleClasses(option: RichOption, isSelected: boolean) {
    if (option.disabled) {
      return "text-sm font-medium text-gray-500";
    } else if (isSelected) {
      return "text-sm font-medium text-white";
    } else {
      return "text-sm font-medium text-gray-300";
    }
  }
  
  function getDescriptionClasses(option: RichOption) {
    return option.disabled ? "text-xs text-gray-600 mt-1" : "text-xs text-gray-400 mt-1";
  }
  
  function handleChange(optionId: string) {
    if (mode === 'radio') {
      selectedValue = optionId;
      onChange(optionId);
    } else {
      if (selectedValues.includes(optionId)) {
        selectedValues = selectedValues.filter(id => id !== optionId);
      } else {
        selectedValues = [...selectedValues, optionId];
      }
      onChange(selectedValues);
    }
  }
  
  // Check if option is selected
  function isSelected(optionId: string): boolean {
    if (mode === 'radio') {
      return selectedValue === optionId;
    } else {
      return selectedValues.includes(optionId);
    }
  }
</script>

<div class="grid grid-cols-1 {columns > 1 ? `md:grid-cols-${columns}` : ''} gap-3">
  {#each options as option}
    {@const selected = isSelected(option.id)}
    <label class={getCardClasses(option, selected)}>
      <input
        type={mode}
        {name}
        value={option.id}
        checked={selected}
        disabled={option.disabled}
        on:change={() => handleChange(option.id)}
        class="mt-0.5 {mode === 'radio' ? 'text-blue-600 bg-gray-700 border-gray-600 focus:ring-blue-500' : 'rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500'}"
      />
      <div class="flex-1 min-w-0">
        <div class={getTitleClasses(option, selected)}>
          {option.title}
          {#if option.category}
            <span class="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-600 text-gray-300">
              {option.category}
            </span>
          {/if}
        </div>
        <div class={getDescriptionClasses(option)}>
          {option.description}
        </div>
      </div>
    </label>
  {/each}
</div>