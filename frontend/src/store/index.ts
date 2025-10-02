import { configureStore } from "@reduxjs/toolkit";
import authReducer from './slices/authSlice';
import messageReducer from './slices/messageSlice';

export const store = configureStore({
  reducer: {
    auth: authReducer,
    message: messageReducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: ['persist/PERSIST', 'persist/REHYDRATE'],
      },
    }),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;