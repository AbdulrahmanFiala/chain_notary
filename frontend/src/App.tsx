import Footer from "@/components/Footer";
import Header from "@/components/Header";
import { message } from "antd";
import { useCallback, useEffect, type FC } from "react";
import { Outlet } from "react-router";
import { useAppDispatch } from "./store/hooks";
import { initializeAuth } from "./store/slices/authSlice";
import { setMessageApi } from "./store/slices/messageSlice";

const App: FC = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const dispatch = useAppDispatch();

  const initSetup = useCallback(async () => {
    await dispatch(initializeAuth());
    await dispatch(setMessageApi(messageApi));
    const messageContent = sessionStorage.getItem("messageApi");

    if (messageContent) {
      messageApi?.warning(messageContent, 10);
      sessionStorage.removeItem("messageApi");
    }
  }, [dispatch, messageApi]);

  useEffect(() => {
    initSetup();
  }, [dispatch, messageApi, initSetup]);

  return (
    <>
      {contextHolder}
      <Header />
      <Outlet />
      <Footer />
    </>
  );
};

export default App;
