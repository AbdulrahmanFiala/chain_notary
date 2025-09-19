import type { ActorSubclass } from "@dfinity/agent";
import type { AuthClient } from "@dfinity/auth-client";
import type { _SERVICE, UserProfile } from "declarations/backend/backend.did";
export interface LoginState {
  actor: ActorSubclass<_SERVICE> | undefined;
  authClient: AuthClient | undefined;
  isAuthenticated: boolean;
  principal: string;
  loading: boolean;
  error: string | null;
  userProfile: UserProfile | null;
}

// Chart data types
export interface ChartDataPoint {
  label: string;
  value: number;
  color: string;
}

export interface ChartConfig {
  title: string;
  type: string;
  data: ChartDataPoint[];
}

export interface ChartsData {
  charts: ChartConfig[];
}
