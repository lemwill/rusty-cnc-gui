<script lang="ts">
	// Your selected Skeleton theme:
	import '@skeletonlabs/skeleton/themes/theme-rocket.css';
	//ùimport "@skeletonlabs/skeleton/themes/theme-skeleton.css";
	// This contains the bulk of Skeletons required styles:
	import '@skeletonlabs/skeleton/styles/all.css';
	import RepeatButton from '../components/RepeatButton.svelte';

	import { AppBar, AppRail, AppRailTile, AppShell, LightSwitch } from '@skeletonlabs/skeleton';

	import Communication from '../lib/Communication.svelte';
	import Toolpath from '../lib/Toolpath.svelte';
	import {
		jog_x_plus,
		jog_x_minus,
		jog_y_plus,
		jog_y_minus,
		jog_z_plus,
		jog_z_minus
	} from '../lib/Communication.svelte';

	import { point_xy_plane, point_xz_plane, point_yz_plane } from '../lib/Toolpath.svelte';
	import { writable, type Writable } from 'svelte/store';

	const storeValue: Writable<number> = writable(0);
</script>

<main>
	<AppShell>
		<svelte:fragment slot="header"><AppBar>Rusty CNC</AppBar></svelte:fragment>
		<svelte:fragment slot="sidebarLeft">
			<AppRail selected={storeValue}>
				<AppRailTile label="Tile 1" value={0}>(icon)</AppRailTile>
				<AppRailTile label="Tile 2" value={1}>(icon)</AppRailTile>
				<AppRailTile label="Tile 3" value={2}>(icon)</AppRailTile>
			</AppRail>
		</svelte:fragment>

		<svelte:fragment slot="sidebarRight">
			<Communication />
			<br />
			<div class="grid grid-cols-3 gap-4">
				<div />
				<div>
					<RepeatButton label="X+" onRepeat={jog_x_plus} />
				</div>
				<div />
				<div>
					<RepeatButton label="Y+" onRepeat={jog_y_plus} />
				</div>
				<div />
				<div>
					<RepeatButton label="Y-" onRepeat={jog_y_minus} />
				</div>
				<div />
				<div>
					<RepeatButton label="X-" onRepeat={jog_x_minus} />
				</div>
				<div />
			</div>
			<br />

			<RepeatButton label="Z+" onRepeat={jog_z_plus} />
			<RepeatButton label="Z-" onRepeat={jog_z_minus} />
		</svelte:fragment>
		<svelte:fragment slot="pageHeader" />
		<!-- Router Slot -->

		<Toolpath />

		<button class="btn variant-filled" on:click={point_xy_plane}>XY Plane</button>

		<button class="btn variant-filled" on:click={point_yz_plane}>YZ Plane</button>
		<button class="btn variant-filled" on:click={point_xz_plane}>XZ Plane</button>
		<LightSwitch />
		<slot />

		<!-- ---- / ---- -->
		<svelte:fragment slot="pageFooter" />
		<svelte:fragment slot="footer" />
	</AppShell>
</main>
