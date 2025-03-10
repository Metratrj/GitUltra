<script lang="ts">
	import * as Menubar from "./components/ui/menubar";
</script>
<Menubar.Root class="rounded-none border-b border-none px-2 lg:px-4">
    <Menubar.Menu>
        <Menubar.Trigger>GitUltra</Menubar.Trigger>
        <Menubar.Content>
            <Menubar.Item>About GitUltra</Menubar.Item>
            <Menubar.Separator />
            <Menubar.Item>
                Preferences...
                <Menubar.Shortcut>⌘,</Menubar.Shortcut>
            </Menubar.Item>
        </Menubar.Content>
    </Menubar.Menu>
</Menubar.Root>