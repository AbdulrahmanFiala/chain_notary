import type { ActorSubclass } from "@dfinity/agent";
import type { AuthClient } from "@dfinity/auth-client";
import type { _SERVICE } from "declarations/backend/backend.did";

export interface LoginState {
  actor: ActorSubclass<_SERVICE> | undefined;
  authClient: AuthClient | undefined;
  isAuthenticated: boolean;
  principal: string;
  loading: boolean;  // Add this for loading states
  error: string | null;  // Add this for error handling
}