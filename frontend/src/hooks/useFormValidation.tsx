import { Form, type FormInstance } from "antd";
import { useEffect, useState } from "react";

const useFormValidation = (form: FormInstance): { isValid: boolean } => {
  const values = Form.useWatch([], form);

  const [isValid, setIsValid] = useState(false);

  useEffect(() => {
    form
      .validateFields({ validateOnly: true })
      .then(() => setIsValid(true))
      .catch(() => setIsValid(false));
  }, [form, values]);

  return { isValid };
}

export default useFormValidation;