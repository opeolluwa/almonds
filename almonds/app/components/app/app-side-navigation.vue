<script setup lang="ts">
import type { Workspace } from "~/stores/workspaces";
import { primaryRoutes, secondaryRoutes } from "~/data/routes";

const workspaceStore = useWorkspacesStore();
const preferenceStore = useUserPreferenceStore();
const showCreateModal = ref(false);
const route = useRoute();
const colorMode = useColorMode();

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});

function toggleTheme() {
  isDark.value = !isDark.value;
}

const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);

const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}

const sidebarCollapsed = ref(false);

const form = reactive({ name: "", description: "" });
const loading = ref(false);
const errors = reactive({ name: "", description: "" });

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

// Shared ref for the newly created workspace across all post-creation steps
const pendingNewWorkspace = ref<Workspace | null>(null);

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  try {
    // Snapshot current profile BEFORE createWorkspace switches activeWorkspaceId
    const existingPref = preferenceStore.preference;
    profileForm.firstName = existingPref?.firstName ?? "";
    profileForm.lastName = existingPref?.lastName ?? "";
    profileForm.email = existingPref?.email ?? "";

    const created = await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    pendingNewWorkspace.value = created;
    showCreateModal.value = false;
    form.name = "";
    form.description = "";
    errors.name = "";
    errors.description = "";
    showProfileSetup.value = true;
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
}

// ── step 1: profile setup ─────────────────────────────────────────────────────
const showProfileSetup = ref(false);
const profileForm = reactive({ firstName: "", lastName: "", email: "" });
const profileSetupLoading = ref(false);

function openSecurityStep() {
  securityMode.value = "new";
  securityPassword.value = "";
  securityConfirm.value = "";
  reuseSourceId.value = "";
  securityError.value = "";
  showSecuritySetup.value = true;
}

async function submitProfileSetup() {
  if (!pendingNewWorkspace.value) return;
  profileSetupLoading.value = true;
  try {
    await preferenceStore.createPreference({
      firstName: profileForm.firstName.trim(),
      lastName: profileForm.lastName.trim(),
      email: profileForm.email.trim(),
    });
  } catch (e) {
    console.error(e);
  } finally {
    profileSetupLoading.value = false;
    showProfileSetup.value = false;
    openSecurityStep();
  }
}

function useDefaultProfile() {
  showProfileSetup.value = false;
  openSecurityStep();
}

// ── step 2: security setup ────────────────────────────────────────────────────
const showSecuritySetup = ref(false);
const securityMode = ref<"new" | "reuse">("new");
const securityPassword = ref("");
const securityConfirm = ref("");
const reuseSourceId = ref("");
const securityError = ref("");
const securitySetupLoading = ref(false);

const securedWorkspaces = computed(() =>
  workspaceStore.workspaces.filter(
    (w) => w.isSecured && w.identifier !== pendingNewWorkspace.value?.identifier,
  ),
);

function closeSecuritySetup() {
  showSecuritySetup.value = false;
  pendingNewWorkspace.value = null;
}

async function submitSecuritySetup() {
  if (!pendingNewWorkspace.value) return;
  securityError.value = "";

  if (securityMode.value === "new") {
    if (!securityPassword.value) {
      securityError.value = "Password is required.";
      return;
    }
    if (securityPassword.value !== securityConfirm.value) {
      securityError.value = "Passwords do not match.";
      return;
    }
  } else {
    if (!reuseSourceId.value || !securityPassword.value) {
      securityError.value = "Select a workspace and enter its password.";
      return;
    }
    const ok = await workspaceStore.verifyWorkspacePassword(
      reuseSourceId.value,
      securityPassword.value,
    );
    if (!ok) {
      securityError.value = "Incorrect password for the selected workspace.";
      return;
    }
  }

  securitySetupLoading.value = true;
  try {
    await workspaceStore.updateWorkspace(pendingNewWorkspace.value.identifier, {
      isSecured: true,
      password: securityPassword.value,
    });
    workspaceStore.unlockWorkspace(pendingNewWorkspace.value.identifier);
    closeSecuritySetup();
  } catch (e) {
    securityError.value = (e as Error).message || "Failed to secure workspace.";
  } finally {
    securitySetupLoading.value = false;
  }
}

const pendingWorkspaceId = ref<string | null>(null);
const showPasswordModal = ref(false);
const passwordInput = ref("");
const passwordError = ref("");
const verifyingPassword = ref(false);

function handleWorkspaceSelect(identifier: string) {
  const ws = workspaceStore.visibleWorkspaces.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  if (ws.isSecured && !workspaceStore.isWorkspaceUnlocked(identifier)) {
    pendingWorkspaceId.value = identifier;
    passwordInput.value = "";
    passwordError.value = "";
    showPasswordModal.value = true;
  } else {
    workspaceStore.setActiveWorkspace(identifier);
  }
}

async function submitPassword() {
  if (!pendingWorkspaceId.value || !passwordInput.value) return;
  verifyingPassword.value = true;
  passwordError.value = "";
  try {
    const ok = await workspaceStore.verifyWorkspacePassword(
      pendingWorkspaceId.value,
      passwordInput.value,
    );
    if (ok) {
      workspaceStore.unlockWorkspace(pendingWorkspaceId.value);
      await workspaceStore.setActiveWorkspace(pendingWorkspaceId.value);
      showPasswordModal.value = false;
      pendingWorkspaceId.value = null;
      passwordInput.value = "";
    } else {
      passwordError.value = "Incorrect password. Please try again.";
    }
  } catch {
    passwordError.value = "Failed to verify password.";
  } finally {
    verifyingPassword.value = false;
  }
}

function closePasswordModal() {
  showPasswordModal.value = false;
  pendingWorkspaceId.value = null;
  passwordInput.value = "";
  passwordError.value = "";
}

const workspaceItems = computed(() => [
  workspaceStore.visibleWorkspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => {
      const isActive = w.identifier === activeId.value;
      return {
        label: w.name,
        icon: isActive
          ? "heroicons:check-circle-solid"
          : w.isSecured && !workspaceStore.isWorkspaceUnlocked(w.identifier)
            ? "heroicons:lock-closed"
            : "heroicons:check-circle",
        class: isActive
          ? "font-semibold text-accent-500 dark:text-accent-400"
          : "text-gray-700 dark:text-gray-300",
        onSelect: () => handleWorkspaceSelect(w.identifier),
      };
    }),
  [
    {
      label: "Manage Workspaces",
      icon: "heroicons:cog-6-tooth",
      class: "text-amber-500 dark:text-amber-400 font-medium",
      onSelect: () => navigateTo("/settings?section=workspaces"),
    },
    {
      label: "Add Workspace",
      icon: "heroicons:plus-circle",
      class: "text-emerald-500 dark:text-emerald-400 font-medium",
      onSelect: () => (showCreateModal.value = true),
    },
  ],
]);

const activeId = computed(() => workspaceStore.currentWorkspace?.identifier);
const activeWorkspaceName = computed(
  () => workspaceStore.currentWorkspace?.name ?? "Select workspace",
);
</script>

<template>
  <UDashboardSidebar
    v-model:collapsed="sidebarCollapsed"
    class="hidden md:flex"
    :collapsible="false"
    :collapsed-size="4"
    :default-size="18"
    :resizable="true"
    :min-size="18"
    :max-size="42"
    :ui="{
      root: 'bg-white dark:bg-gray-900 overflow-y-scroll transition-[width] duration-300 border-e border-gray-200 dark:border-gray-800',
      header: 'shrink-0 h-auto p-0',
      body: 'flex-1 overflow-y-scroll scrollbar-config p-0 gap-0 ',
      footer: 'shrink-0 h-auto p-0',
    }"
  >
    <!-- Sidebar body: primary nav -->
    <template #default="{ collapsed }">
      <UDropdownMenu
        :items="workspaceItems"
        :ui="{
          content:
            'min-w-52 rounded-xl shadow-xl border border-gray-100 dark:border-gray-800 py-1.5',
          item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
          separator: 'my-1 mx-2',
        }"
      >
        <button
          class="flex items-center gap-2.5 w-full px-3 py-2 mt-2 mb-6 rounded-xl bg-gray-50 dark:bg-gray-800/60 hover:bg-gray-100 dark:hover:bg-gray-800 border border-gray-200 dark:border-gray-700 transition-colors group"
        >
          <span
            class="flex items-center justify-center size-6 rounded-md bg-accent-100 dark:bg-accent-900 shrink-0"
          >
            <UIcon
              name="heroicons:briefcase"
              class="size-3.5 text-accent-600 dark:text-accent-400"
            />
          </span>
          <span
            class="flex-1 text-left text-sm font-medium text-gray-800 dark:text-gray-200 truncate"
          >
            {{ activeWorkspaceName }}
          </span>
          <UIcon
            name="heroicons:chevron-up-down"
            class="size-3.5 text-gray-400 dark:text-gray-500 shrink-0 group-hover:text-gray-600 dark:group-hover:text-gray-300 transition-colors"
          />
        </button>
      </UDropdownMenu>

      <div class="flex flex-col gap-0.5 px-2 py-2 overflow-y-scroll">
        <NuxtLink
          v-for="r in primaryRoutes"
          :key="r.name"
          :to="r.path"
          class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
          :class="[
            collapsed ? 'justify-center' : 'gap-3',
            isActive(r.path)
              ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800',
          ]"
        >
          <UIcon
            :name="isActive(r.path) ? r.activeIcon : r.icon"
            class="size-4 shrink-0"
          />
          <span v-if="!collapsed">{{ r.name }}</span>
        </NuxtLink>
      </div>
    </template>

    <!-- Sidebar footer: theme + secondary nav -->
    <template #footer="{ collapsed }">
      <div class="flex flex-col gap-0.5 px-2 pb-4 w-full mb-12">
        <USeparator class="mx-1 mb-2" />

        <button
          class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
          :class="collapsed ? 'justify-center' : 'gap-3'"
          @click="toggleTheme"
        >
          <UIcon :name="themeIcon" class="size-4 shrink-0" />
          <span v-if="!collapsed">{{ themeLabel }}</span>
        </button>

        <NuxtLink
          v-for="r in secondaryRoutes"
          :key="r.name"
          :to="r.path"
          class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
          :class="[
            collapsed ? 'justify-center' : 'gap-3',
            isActive(r.path)
              ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800',
          ]"
        >
          <UIcon
            :name="isActive(r.path) ? r.activeIcon : r.icon"
            class="size-4 shrink-0"
          />
          <span v-if="!collapsed">{{ r.name }}</span>
        </NuxtLink>
      </div>
    </template>
  </UDashboardSidebar>
  <!-- Password prompt for secured workspaces -->
  <UModal v-model:open="showPasswordModal" @close="closePasswordModal">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Secured Workspace
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Enter the password to access this workspace.
        </p>
      </div>
      <form
        class="px-6 pb-6 mt-4 flex flex-col gap-4"
        @submit.prevent="submitPassword"
      >
        <AppInput
          v-model="passwordInput"
          label="Password"
          type="password"
          name="workspace-password"
          placeholder="Enter password"
          :error="passwordError"
          :disabled="verifyingPassword"
          autofocus
        />
        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="verifyingPassword"
            @click="closePasswordModal"
          >
            Cancel
          </UButton>
          <UButton
            type="submit"
            color="primary"
            :loading="verifyingPassword"
            :disabled="!passwordInput || verifyingPassword"
          >
            Unlock
          </UButton>
        </div>
      </form>
    </template>
  </UModal>

  <!-- Profile setup modal (step 1 after workspace creation) -->
  <UModal :open="showProfileSetup" @close="useDefaultProfile">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Set up your profile
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Add a profile for <span class="font-medium text-gray-700 dark:text-gray-200">{{ pendingNewWorkspace?.name }}</span>, or continue with your existing details.
        </p>
      </div>

      <div class="px-6 pb-6 mt-4 flex flex-col gap-4">
        <div class="grid grid-cols-2 gap-3">
          <AppInput
            v-model="profileForm.firstName"
            label="First name"
            type="text"
            name="profile-first-name"
            placeholder="John"
            :disabled="profileSetupLoading"
          />
          <AppInput
            v-model="profileForm.lastName"
            label="Last name"
            type="text"
            name="profile-last-name"
            placeholder="Doe"
            :disabled="profileSetupLoading"
          />
        </div>
        <AppInput
          v-model="profileForm.email"
          label="Email"
          type="email"
          name="profile-email"
          placeholder="john@example.com"
          :disabled="profileSetupLoading"
        />

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="profileSetupLoading"
            @click="useDefaultProfile"
          >
            Continue with default
          </UButton>
          <UButton
            type="button"
            color="primary"
            :loading="profileSetupLoading"
            :disabled="!profileForm.firstName.trim() || !profileForm.email.trim() || profileSetupLoading"
            @click="submitProfileSetup"
          >
            Save profile
          </UButton>
        </div>
      </div>
    </template>
  </UModal>

  <!-- Security setup modal (step 2 after workspace creation) -->
  <UModal :open="showSecuritySetup" @close="closeSecuritySetup">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Secure "{{ pendingNewWorkspace?.name }}"
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Optionally protect this workspace with a password.
        </p>
      </div>

      <div class="px-6 pb-6 mt-4 flex flex-col gap-4">
        <!-- Mode selector -->
        <div class="flex gap-2">
          <button
            class="flex-1 rounded-lg border px-3 py-2 text-xs font-medium transition-colors"
            :class="
              securityMode === 'new'
                ? 'border-accent-500 bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300'
                : 'border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
            "
            @click="securityMode = 'new'; securityPassword = ''; securityConfirm = ''; securityError = ''"
          >
            New password
          </button>
          <button
            v-if="securedWorkspaces.length > 0"
            class="flex-1 rounded-lg border px-3 py-2 text-xs font-medium transition-colors"
            :class="
              securityMode === 'reuse'
                ? 'border-accent-500 bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300'
                : 'border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
            "
            @click="securityMode = 'reuse'; securityPassword = ''; securityConfirm = ''; reuseSourceId = securedWorkspaces[0]?.identifier ?? ''; securityError = ''"
          >
            Reuse existing profile
          </button>
        </div>

        <!-- New password fields -->
        <template v-if="securityMode === 'new'">
          <AppInput
            v-model="securityPassword"
            label="Password"
            type="password"
            name="new-ws-password"
            placeholder="Enter password"
            :disabled="securitySetupLoading"
          />
          <AppInput
            v-model="securityConfirm"
            label="Confirm password"
            type="password"
            name="new-ws-password-confirm"
            placeholder="Confirm password"
            :disabled="securitySetupLoading"
          />
        </template>

        <!-- Reuse from existing workspace -->
        <template v-else>
          <UFormField label="Copy from workspace">
            <USelect
              v-model="reuseSourceId"
              :items="securedWorkspaces.map((w) => ({ label: w.name, value: w.identifier }))"
              class="w-full"
              :disabled="securitySetupLoading"
            />
          </UFormField>
          <AppInput
            v-model="securityPassword"
            :label="`Password for &quot;${securedWorkspaces.find((w) => w.identifier === reuseSourceId)?.name ?? 'workspace'}&quot;`"
            type="password"
            name="reuse-ws-password"
            placeholder="Enter that workspace's password"
            :disabled="securitySetupLoading"
          />
        </template>

        <p v-if="securityError" class="text-xs text-red-500 dark:text-red-400">
          {{ securityError }}
        </p>

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="securitySetupLoading"
            @click="closeSecuritySetup"
          >
            Skip
          </UButton>
          <UButton
            type="button"
            color="primary"
            :loading="securitySetupLoading"
            :disabled="!securityPassword || securitySetupLoading"
            @click="submitSecuritySetup"
          >
            Secure workspace
          </UButton>
        </div>
      </div>
    </template>
  </UModal>

  <UModal v-model:open="showCreateModal">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Create a New Workspace
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Set up a new workspace to organize your projects and files. You can
          have multiple workspaces and switch between them easily.
        </p>
      </div>

      <form
        class="px-6 pb-6 mt-4 flex flex-col gap-4"
        @submit.prevent="handleSubmit"
      >
        <div class="grid grid-cols-2 gap-3">
          <AppInput
            v-model="form.name"
            label="Name"
            hint="required"
            type="text"
            name="workspace-name"
            placeholder="Almonds"
            :error="errors.name"
            :disabled="loading"
          />
          <AppInput
            v-model="form.description"
            label="Description"
            hint="required"
            type="text"
            name="workspace-description"
            placeholder="Organize files and tasks"
            :error="errors.description"
            :disabled="loading"
          />
        </div>

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="loading"
            @click="showCreateModal = false"
          >
            Cancel
          </UButton>
          <UButton
            type="submit"
            color="primary"
            :loading="loading"
            :disabled="loading"
          >
            Save and continue
          </UButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
