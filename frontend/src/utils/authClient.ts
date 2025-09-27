import { AuthClient } from "@dfinity/auth-client";
import { canisterId, createActor } from "declarations/backend";

export const getAuthClient = async () => {
  const authClient = await AuthClient.create();
  const identity = authClient.getIdentity();
  const actor = createActor(canisterId, {
    agentOptions: {
      identity,
    },
  });
  return { authClient, actor, identity };
};
