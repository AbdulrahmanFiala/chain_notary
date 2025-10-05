import useFormValidation from "@/hooks/useFormValidation";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { register } from "@/store/slices/authSlice";
import { Button, Card, Form, Input, Typography } from "antd";
import { useState, type FC } from "react";
import { Navigate, useNavigate } from "react-router";

const { Title, Text } = Typography;

const UserRegistration: FC = () => {
  const [form] = Form.useForm();
  const { actor, isAuthenticated, userProfile } = useAppSelector(
    (state) => state.auth,
  );
  const { messageApi } = useAppSelector((state) => state.message);
  const dispatch = useAppDispatch();
  const [isLoading, setIsLoading] = useState(false);

  const navigate = useNavigate();

  const { isValid } = useFormValidation(form);

  const handleSubmit = async ({
    name,
    email,
  }: {
    name: string;
    email: string;
  }) => {
    setIsLoading(true);
    if (!actor) {
      messageApi?.error("Authentication required");
      return;
    }

    try {
      await dispatch(register({ name, email })).unwrap();
      navigate("/");
      messageApi?.success("Registration successful");
    } catch (err) {
      messageApi?.error(
        err instanceof Error ? err.message : "Registration failed",
      );
    } finally {
      setIsLoading(false);
    }
  };

  if (isAuthenticated && userProfile?.name && userProfile?.email)
    return <Navigate to="/" state={{ from: location }} replace />;
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4">
      <Card className="w-full max-w-md">
        <div className="text-center mb-6">
          <Title level={2}>Complete Your Registration</Title>
          <Text type="secondary">Please provide your details to continue</Text>
        </div>

        <Form
          form={form}
          layout="vertical"
          onFinish={handleSubmit}
          size="large"
        >
          <Form.Item
            name="name"
            label="Full Name"
            rules={[
              { required: true, message: "Please enter your full name" },
              { min: 2, message: "Name must be at least 2 characters" },
              { max: 50, message: "Name must be at most 50 characters" },
              {
                pattern: /^[a-zA-Z\s]+$/,
                message: "Name must contain only letters and spaces",
              },
              {
                whitespace: true,
                message: "Name cannot be just whitespace",
              },
            ]}
          >
            <Input placeholder="Full Name" />
          </Form.Item>

          <Form.Item
            name="email"
            label="Email Address"
            rules={[
              { required: true, message: "Please enter your email" },
              { type: "email", message: "Please enter a valid email" },
            ]}
          >
            <Input placeholder="Email Address" />
          </Form.Item>

          <Form.Item>
            <Button
              disabled={(isValid && isLoading) || !isValid}
              loading={isLoading}
              type="primary"
              htmlType="submit"
              block
            >
              Complete Registration
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
};

export default UserRegistration;
