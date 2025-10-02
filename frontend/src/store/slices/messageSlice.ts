import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import type { MessageInstance } from 'antd/es/message/interface'

interface MessageState {
  messageApi: MessageInstance | null
}

const initialState: MessageState = {
  messageApi: null
}

const messageSlice = createSlice({
  name: 'message',
  initialState,
  reducers: {
    setMessageApi: (state, action: PayloadAction<MessageInstance>) => {
      state.messageApi = action.payload
    }
  }
})

export const { setMessageApi } = messageSlice.actions
export default messageSlice.reducer