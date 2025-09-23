import type { LoginState } from "@/interfaces";
import { AuthClient } from "@dfinity/auth-client";
import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { canisterId, createActor } from "declarations/backend";
import type { UserProfile } from "declarations/backend/backend.did";

const initialState: LoginState = {
  actor: undefined,
  authClient: undefined,
  isAuthenticated: false,
  principal: "",
  loading: false,
  error: null,
  userProfile: null,
};

const network = process.env.DFX_NETWORK;
const identityProvider =
  network === "ic"
    ? "https://identity.ic0.app" // Mainnet
    : "http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:8080"; // Local

// Async Thunks (for async operations)
export const updateActor = createAsyncThunk(
  "auth/updateActor",
  async (_, { rejectWithValue }) => {
    const authClient = await AuthClient.create();
    const identity = authClient.getIdentity();
    const actor = createActor(canisterId, {
      agentOptions: {
        identity,
      },
    });
    try {
      const result = await actor?.whoami();
      const principal = result?.toString() || "";
      const isAuthenticated = await authClient.isAuthenticated();

      const data = await actor?.get_user_profile();
      let userProfile: UserProfile | null = null;
      if ("Ok" in data && data.Ok.length > 0) {
        userProfile = data.Ok[0] || null;
      }

      return {
        actor,
        authClient,
        isAuthenticated,
        principal,
        userProfile: userProfile || null,
      };
    } catch (error) {
      console.error("Error updating actor:", error);
      return rejectWithValue(
        error instanceof Error ? error.message : "Unknown error",
      );
    }
  },
);

export const login = createAsyncThunk(
  "auth/login",
  async (_, { dispatch, rejectWithValue }) => {
    try {
      const authClient = await AuthClient.create();
      return new Promise<string>((resolve, reject) => {
        authClient.login({
          identityProvider,
          onSuccess: async () => {
            try {
              // After successful login, update the actor
              await dispatch(updateActor()).unwrap();
              resolve("Login successful");
            } catch (error) {
              reject(error);
            }
          },
          onError: (error) => {
            reject(error || "Login failed");
          },
        });
      });
    } catch (error) {
      console.error("Login error:", error);
      return rejectWithValue(
        error instanceof Error ? error.message : "Login failed",
      );
    }
  },
);

export const register = createAsyncThunk(
  "auth/register",
  async (
    { name, email }: { name: string; email: string },
    { dispatch, rejectWithValue },
  ) => {
    try {
      const authClient = await AuthClient.create();
      const identity = authClient.getIdentity();
      const actor = createActor(canisterId, {
        agentOptions: {
          identity,
        },
      });
      const data = await actor.register_user(name, email);
      if ("Ok" in data) {
        return data.Ok;
      }

      // After successful registration, update the actor
      await dispatch(updateActor()).unwrap();
      return "Register successful";
    } catch (error) {
      console.error("Register error:", error);
      return rejectWithValue(
        error instanceof Error ? error.message : "Register failed",
      );
    }
  },
);

export const logout = createAsyncThunk(
  "auth/logout",
  async (_, { getState, dispatch, rejectWithValue }) => {
    try {
      const state = getState() as { auth: LoginState };
      const { authClient } = state.auth;

      if (authClient) {
        await authClient.logout();
      }

      // Update actor after logout
      await dispatch(updateActor()).unwrap();

      return "Logout successful";
    } catch (error) {
      console.error("Logout error:", error);
      return rejectWithValue(
        error instanceof Error ? error.message : "Logout failed",
      );
    }
  },
);

// Initialize auth on app start
export const initializeAuth = createAsyncThunk(
  "auth/initialize",
  async (_, { dispatch }) => {
    await dispatch(updateActor());
  },
);

const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    // Synchronous reducers only
    clearError: (state) => {
      state.error = null;
    },
    setPrincipal: (state, action) => {
      state.principal = action.payload;
    },
    reset: (state) => {
      state.actor = undefined;
      state.authClient = undefined;
      state.isAuthenticated = false;
      state.principal = "";
      state.loading = false;
      state.error = null;
      state.userProfile = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Update Actor
      .addCase(updateActor.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(updateActor.fulfilled, (state, action) => {
        state.loading = false;
        state.actor = action.payload.actor;
        state.authClient = action.payload.authClient;
        state.isAuthenticated = action.payload.isAuthenticated;
        state.principal = action.payload.principal;
        state.userProfile = action.payload.userProfile;
      })
      .addCase(updateActor.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })

      // Login
      .addCase(login.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(login.fulfilled, (state) => {
        state.loading = false;
        // Actor will be updated by updateActor thunk
      })
      .addCase(login.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })

      // Logout
      .addCase(logout.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(logout.fulfilled, (state) => {
        state.loading = false;
        // State will be updated by updateActor thunk
      })
      .addCase(logout.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })

      // Initialize
      .addCase(initializeAuth.pending, (state) => {
        state.loading = true;
      })
      .addCase(initializeAuth.fulfilled, (state) => {
        state.loading = false;
      })
      .addCase(initializeAuth.rejected, (state) => {
        state.loading = false;
        state.error = "Failed to initialize authentication";
      });
  },
});

export const { clearError, setPrincipal, reset } = authSlice.actions;
export default authSlice.reducer;
