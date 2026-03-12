<script lang="ts">
  import { tagStore } from "./tags.svelte";
  import type { TagGroup, Tag } from "./tags.svelte";
  import { imageStore } from "./images.svelte";
  import { configStore } from "./config.svelte";

  let open = $state(false);
  let pendingTag = $state("");
  let newGroupName = $state("");
  let newGroupAutoTaggable = $state(true);
  let selectedGroupId = $state<number | null>(null);

  const toggleOpen = () => {
    open = !open;
  };

  const addTag = async () => {
    if (selectedGroupId !== null) {
        await tagStore.createTagWithGroup(pendingTag, selectedGroupId);
    } else {
        await tagStore.create(pendingTag);
    }
    pendingTag = "";
  };

  const createGroup = async () => {
    if (newGroupName) {
      await tagStore.createGroup(newGroupName, newGroupAutoTaggable);
      newGroupName = "";
    }
  };

  const assignTagToGroup = async (tagName: string, groupId: number | null) => {
    await tagStore.assignToGroup(tagName, groupId);
  };

  $effect.pre(() => {
    tagStore.refresh();
  });

  let groupedTags = $derived.by(() => {
    const groups: Record<number | "other", { group: TagGroup | null, tags: Tag[] }> = {
        "other": { group: null, tags: [] }
    };
    
    tagStore.groups.forEach(g => {
        groups[g.id] = { group: g, tags: [] };
    });

    tagStore.tagObjects.filter(t => 
        t.name.toLowerCase().includes(pendingTag.toLowerCase())
    ).forEach(t => {
        if (t.groupId && groups[t.groupId]) {
            groups[t.groupId].tags.push(t);
        } else {
            groups["other"].tags.push(t);
        }
    });

    return Object.values(groups).filter(g => g.tags.length > 0 || g.group !== null);
  });
</script>

<button onclick={toggleOpen}>
  <span class="toggle-triangle">{open ? "▼" : "▶"}</span>
  Tags & Groups
</button>

{#if open}
  <div class="tag-menu">
    <button onclick={tagStore.refresh}>Refresh</button>
    
    <div class="clip-actions">
      <button
        onclick={() =>
          tagStore.clipAutoTag(
            imageStore.filteredImages.map((i) => i.path),
            configStore.clipThreshold
          )}>CLIP Auto-Tag All</button
      >
      <button
        onclick={() =>
          tagStore.clipAutoTag(
            imageStore.selectedImages.map((i) => i.path),
            configStore.clipThreshold
          )}>CLIP Auto-Tag Selected</button
      >
    </div>

    <div class="group-creator">
        <input type="text" placeholder="New Group Name" bind:value={newGroupName} />
        <label>
            Auto-taggable
            <input type="checkbox" bind:checked={newGroupAutoTaggable} />
        </label>
        <button onclick={createGroup}>Create Group</button>
    </div>

    <div class="taglist">
      {#each groupedTags as { group, tags }}
        <div class="tag-group-section">
          <div class="group-header">
            <h3>{group ? group.name : "Other"}</h3>
            {#if group}
                <label class="auto-tag-toggle">
                    Auto
                    <input 
                        type="checkbox" 
                        checked={group.isAutoTaggable} 
                        onchange={(e) => tagStore.updateGroupAutoTaggable(group.id, e.currentTarget.checked)}
                    />
                </label>
            {/if}
          </div>
          {#each tags as tag}
            <div class="tag-item">
              <span class="tag-name">{tag.name}</span>
              <div class="tag-actions">
                <button
                    onclick={() =>
                    tagStore.tagAllImages(
                        tag.name,
                        imageStore.filteredImages.map((i) => i.path)
                    )}>All</button
                >
                <button
                    onclick={() =>
                    tagStore.tagAllImages(
                        tag.name,
                        imageStore.selectedImages.map((i) => i.path)
                    )}>Sel</button
                >
                <select 
                    value={tag.groupId ?? ""} 
                    onchange={(e) => assignTagToGroup(tag.name, e.currentTarget.value ? parseInt(e.currentTarget.value) : null)}
                >
                    <option value="">None</option>
                    {#each tagStore.groups as g}
                        <option value={g.id}>{g.name}</option>
                    {/each}
                </select>
              </div>
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <div class="tag-creator">
        <input type="text" autocomplete="off" placeholder="New Tag Name" bind:value={pendingTag} />
        <select bind:value={selectedGroupId}>
            <option value={null}>No Group</option>
            {#each tagStore.groups as g}
                <option value={g.id}>{g.name}</option>
            {/each}
        </select>
        <button onclick={addTag}>Add Tag</button>
    </div>
  </div>
{/if}

<style>
  .tag-group-section {
    border-bottom: 1px solid #444;
    padding-bottom: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .group-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .auto-tag-toggle {
    font-size: 0.7rem;
    color: #888;
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .tag-group-section h3 {
    margin: 0.5rem 0;
    font-size: 0.9rem;
    color: #aaa;
  }
  .tag-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 2px 5px;
    background: #6d6c6c;
    margin-bottom: 2px;
    border-radius: 4px;
  }
  .tag-name {
    flex-grow: 1;
  }
  .tag-actions {
    display: flex;
    gap: 4px;
  }
  .taglist {
    display: flex;
    flex-direction: column;
    padding: 0 5%;
    height: 40vh;
    overflow-y: scroll;
    border: 1px solid #333;
    border-radius: 4px;
  }
  .toggle-triangle {
    display: inline-block;
    width: 1rem;
    height: 1rem;
  }
  .tag-menu {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 0.5rem 5%;
  }
  .clip-actions, .group-creator, .tag-creator {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.5rem;
    border: 1px solid #333;
    border-radius: 8px;
    background: #6d6c6c;
  }
  h3 {
    text-align: left;
  }
</style>
