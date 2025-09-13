# Memory Wipe Tracking System

## Overview

This system provides comprehensive tracking and logging for memory wipes in your Internet Computer canister. It addresses the critical issue where memory gets wiped and you need to track when and how this happens.

## Key Features

### 1. **Persistent Logging**
- Logs are written to IC system logs which persist across upgrades
- Logs include timestamps, canister IDs, and detailed event information
- Multiple logging levels: INFO, WARNING, CRITICAL

### 2. **Memory Wipe Detection**
- Automatic detection during canister upgrades
- Manual memory wipe checks via API calls
- Anomaly detection for partial memory wipes
- Storage integrity validation

### 3. **External Logging Support**
- Webhook integration for Discord.
- Structured JSON logging format
- Configurable external services

## How It Works

### The Problem
When memory gets wiped in your IC canister, any logs stored in memory would also be wiped. This creates a chicken-and-egg problem where you can't track memory wipes because the tracking logs get wiped too.

### The Solution
This system uses **IC system logs** (`ic_cdk::println!`) which persist outside of your canister's memory and survive memory wipes. These logs are accessible through the IC dashboard and can be monitored externally.

## Usage

### 1. Check Memory Wipe Status

```bash
# Query current memory status
dfx canister call backend get_memory_wipe_logs
```

This returns:
- Current storage statistics
- Whether memory appears to be wiped
- Instructions for checking logs
- Timestamp of last check

### 2. Manual Memory Wipe Check

```bash
# Manually trigger memory wipe detection
dfx canister call backend check_for_memory_wipe
```

### 3. Monitor IC System Logs

The system automatically logs events with these prefixes:
- `MEMORY_EVENT:` - General memory events
- `MEMORY_EVENT_DATA:` - Detailed data for events
- `MEMORY_ANOMALY:` - Detected anomalies
- `EXTERNAL_LOG:` - Structured external logs


## Event Types

### Critical Events
- `POTENTIAL_MEMORY_WIPE` - All storage counts are zero
- `MEMORY_WIPE_DETECTED` - Confirmed memory wipe

### Warning Events
- `MEMORY_ANOMALY` - Suspicious storage patterns
- `MANUAL_MEMORY_WIPE_CHECK` - Manual check triggered

### Info Events
- `CANISTER_INIT` - Canister initialization
- `PRE_UPGRADE` - Before canister upgrade
- `POST_UPGRADE` - After canister upgrade
- `POST_UPGRADE_FINAL` - Final state after upgrade


## Troubleshooting

### If Memory Gets Wiped

1. **Check IC System Logs**
   - Look for `POTENTIAL_MEMORY_WIPE` events
   - Check timestamps to determine when it happened
   - Look for patterns in upgrade events

2. **Check External Logs**
   - If you have webhooks configured, check Discord/Slack
   - Look for external logging service notifications

3. **Investigate Root Cause**
   - Check if it happened during an upgrade
   - Look for any error messages in logs
   - Check IC network status at the time


## Implementation Details

The system works by:

1. **Logging to IC System Logs**: Uses `ic_cdk::println!` which persists outside memory
2. **Event Tracking**: Logs all critical events with structured data
3. **Anomaly Detection**: Checks for suspicious patterns in storage
4. **External Integration**: Supports webhooks and APIs for external logging
5. **Upgrade Monitoring**: Tracks memory state before and after upgrades

This ensures that even if your canister's memory gets wiped, you'll have a persistent record of what happened and when.
