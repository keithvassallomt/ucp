# UCP Design Brief & Functional Overview

## 1. Product Summary
**UCP (Universal Copy Paste)** is a privacy-focused, serverless tool that synchronizes the clipboard between devices on the same local network.
*   **Goal**: Copy text on your laptop, paste it immediately on your desktop.
*   **Key Differentiator**: It is completely decentralized (Peer-to-Peer). No cloud, no accounts, no login. Security is handled via a shared "Cluster Key" derived from a PIN.

## 2. Core Mental Model: The "Secure Cluster"
Unlike Bluetooth pairing (1-to-1), UCP uses a **Cluster** model.
*   **The Network**: A group of trusted devices (e.g., "Keith's Devices").
*   **Admission**: To join the network, a new device must enter the **Network PIN** from *any* existing device in the cluster.
*   **Shared State**: Once joined, the device receives the "Cluster Key" and automatically discovers/trusts all other peers in that cluster.
*   **Isolation**: Devices can leave the cluster (resetting themselves) or be kicked (banned) by others.

---

## 3. UI Inventory & Functional Requirements

The current application has two main views controlled by a tab/toggle in the header: **Devices** and **History**.

### A. Global Header
*   **App Status**: Indicates the app is running (currently a simple green dot).
*   **Navigation**: Toggles between "Devices" (Network Management) and "History" (Clipboard Content).
*   **Leave Network Button**:
    *   **Function**: A destructive action. It wipes the device's identity, keys, and trusted peers, effectively "factory resetting" the network state.
    *   **UX**: Requires a confirmation dialog.
    *   **Result**: The app reloads as a fresh, generic device ready to join a new network.

### B. "Devices" View (Network Manager)
This is the command center for connections. Ideally, users set this up once and rarely touch it.

#### 1. "My Device" Card (Identity)
*   **Status Information**:
    *   **My Network Name**: The random name of the cluster I am currently in (e.g., `active-falcon`).
    *   **My Device ID**: My unique name (e.g., `ucp-1234`).
    *   **Network PIN**: The 6-character secret code (e.g., `AB12CD`) that *other* devices need to enter to join *me*.
*   **Action**: This information must be highly visible for pairing new devices.

#### 2. "Trusted Peers" List (My Cluster)
*   **Content**: A list of verified devices currently in my secure network.
*   **Status Indicators**:
    *   Online/Offline status (based on mDNS discovery).
*   **Actions per Peer**:
    *   **Kick / Ban (Trash Icon)**: Permanently removes the peer from the network. This sends a "Destruct" command to that peer, forcing it to reset.

#### 3. "Nearby Networks" List (Discovery)
*   **Content**: A list of *other* UCP devices on the LAN that are NOT in my cluster.
*   **Grouping**: Grouped by their advertised Network Name (e.g., "Groups of devices available to join").
*   **Actions**:
    *   **Join Button**: Initiates the pairing handshake.
    *   **Flow**:
        1.  User clicks "Join" next to a network (e.g., `reticent-monkey`).
        2.  **PIN Dialog**: User is prompted to enter the PIN displayed on *that* network's devices.
        3.  **Success**: Device receives keys and moves its peers to the "Trusted" list.
        4.  **Failure**: Invalid PIN or timeout.

### C. "History" View (Clipboard Manager)
This is the day-to-day utility view.
*   **Content**: A chronological list of recent clipboard entries (local copies and remote receives).
*   **Items Display**:
    *   Text preview of the content.
    *   Device origin (did I copy this, or did `ucp-5678` send it?).
    *   Timestamp.
*   **Actions per Item**:
    *   **Copy**: Re-injects the item into the current clipboard (useful for retrieving older items).
    *   **Delete**: Removes the item from history.
*   **Global Actions**:
    *   **Clear History**: Wipes all stored logs.

## 4. UX Challenges for the Designer
1.  **"Zero State" vs "Joined State"**:
    *   When a user first installs, they are in a "Network of One" (random name, generated PIN).
    *   They need clear guidance: "Share this PIN to add devices" OR "Look below to join an existing network".
2.  **Security Visibility**:
    *   How do we communicate that the connection is encrypted?
    *   How do we prevent accidental "Leaving"?
3.  **Cross-Platform Constraints**:
    *   **macOS / Windows**: The UI must look native-ish or at least premium on both.
    *   **Window Controls**: The design must account for the draggable title bar area (avoid putting clickable elements in the top-left/top-right corners where window controls live, or ensure they are marked non-draggable).
