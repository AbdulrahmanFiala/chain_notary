import useFormValidation from "@/hooks/useFormValidation";
import createInstitutionForUserService from "@/services/admin/createInstitutionForUser.service";
import getAllUsersService from "@/services/admin/getAllUsers.service";
import linkUserToInstitutionService from "@/services/admin/linkUserToInstitution.service";
import promoteToSuperAdminService from "@/services/admin/promoteToSuperAdmin.service";
import unlinkUserFromInstitutionService from "@/services/admin/unlinkUserFromInstitution.service";
import getAllInstitutions from "@/services/institutions/getAllInstitutions";
import { useAppSelector } from "@/store/hooks";
import type { Principal } from "@dfinity/principal";
import type { TableProps } from "antd";
import { Button, Form, Input, Modal, Select, Switch, Table, Tag } from "antd";
import type { UserProfile } from "declarations/backend/backend.did";
import { startCase } from "lodash";
import { useCallback, useEffect, useState, type FC } from "react";

const UsersTable: FC = () => {
  const [open, setOpen] = useState<boolean>(false);
  const [loadingModal, setLoadingModal] = useState<boolean>(false);
  const [selectedUser, setSelectedUser] = useState<Principal | null>(null);
  const [isNewInstitution, setIsNewInstitution] = useState(false);
  const [institutions, setInstitutions] = useState<
    { value: string; label: string }[]
  >([]);
  const [users, setUsers] = useState<UserProfile[]>([]);
  const [loading, setLoading] = useState(false);

  const { messageApi } = useAppSelector((state) => state.message);
  const [form] = Form.useForm();
  const { isValid } = useFormValidation(form);

  const fetchUsers = useCallback(async () => {
    setLoading(true);
    try {
      const response = await getAllUsersService();
      setUsers(response);
    } catch {
      messageApi?.error("Error fetching users");
    } finally {
      setLoading(false);
    }
  }, [messageApi]);

  const fetchInstitutions = useCallback(async () => {
    setOpen(true);
    setLoadingModal(true);
    try {
      const data = await getAllInstitutions();
      setInstitutions(
        data.map((institution) => ({
          value: institution.institution_id,
          label: institution.name,
        })),
      );
    } catch {
      setOpen(false);
      messageApi?.error("Error fetching institutions");
    } finally {
      setLoadingModal(false);
    }
  }, [messageApi]);

  const columns: TableProps<UserProfile>["columns"] = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      filtered: true,
    },
    {
      title: "Email",
      dataIndex: "email",
      key: "email",
      filtered: true,
    },
    {
      title: "Internet Identity",
      dataIndex: "internet_identity",
      key: "internet_identity",
      ellipsis: true,
      render: (internet_identity) => internet_identity.toString(),
      width: 100,
    },
    {
      title: "Role",
      dataIndex: "role",
      key: "role",
      render: (role) => {
        const roleKey = Object.keys(role)[0];
        return (
          <Tag
            color={
              roleKey === "SuperAdmin"
                ? "red"
                : roleKey === "RegularUser"
                  ? "blue"
                  : "gold"
            }
          >
            {startCase(roleKey)}
          </Tag>
        );
      },
    },
    {
      title: "Institution ID",
      dataIndex: "assigned_institution_id",
      key: "assigned_institution_id",
      render: (institution_id) => (institution_id ? institution_id : "N/A"),
    },
    {
      title: "Created At",
      dataIndex: "created_at",
      key: "created_at",
      render: (timestamp) =>
        new Date(Number(timestamp) / 1000000).toLocaleDateString(),
    },
    {
      title: "Last Login",
      dataIndex: "last_login",
      key: "last_login",
      render: (timestamp) =>
        timestamp
          ? new Date(Number(timestamp) / 1000000).toLocaleDateString()
          : "Never",
    },
    {
      title: "Actions",
      key: "actions",
      children: [
        {
          title: "Promote",
          render: (_, record) => (
            <Switch
              disabled={Object.keys(record.role)[0] === "SuperAdmin" || loading}
              size="small"
              checked={Object.keys(record.role)[0] === "SuperAdmin"}
              onClick={async () => {
                if (Object.keys(record.role)[0] === "SuperAdmin") return;
                setLoading(true);
                try {
                  await promoteToSuperAdminService(record.internet_identity);
                  messageApi?.success("User promoted to SuperAdmin");
                } catch {
                  messageApi?.error("Error promoting user");
                  setLoading(false);
                }
                fetchUsers();
              }}
            />
          ),
        },
        {
          title: "Assign",
          render: (_, record) => (
            <Switch
              disabled={loading}
              size="small"
              checked={!!record.assigned_institution_id}
              onClick={async () => {
                if (!record.assigned_institution_id) {
                  setSelectedUser(record.internet_identity);
                  fetchInstitutions();
                } else {
                  setLoading(true);
                  try {
                    await unlinkUserFromInstitutionService(
                      record.internet_identity,
                    );
                    messageApi?.success("User unlinked from institution");
                  } catch {
                    messageApi?.error("Error unlinking user from institution");
                    setLoading(false);
                  }
                }

                fetchUsers();
              }}
            >
            </Switch>
          ),
        },
      ],
      fixed: "right",
    },
  ];

  useEffect(() => {
    fetchUsers();
  }, [fetchUsers]);

  return (
    <>
      <Table<UserProfile>
        bordered
        columns={columns}
        dataSource={users}
        loading={loading}
        scroll={{ x: "max-content" }}
      />
      <Modal
        title="Link User to Institution"
        centered
        open={open}
        confirmLoading={loading}
        loading={loadingModal}
        maskClosable={false}
        closable={false}
        destroyOnHidden
        onOk={async () => {
          if (isValid && selectedUser) {
            const formValues = form.getFieldsValue();

            setLoading(true);
            setOpen(false);
            try {
              if (isNewInstitution) {
                await createInstitutionForUserService(
                  selectedUser,
                  formValues.newInstitution,
                  formValues.email,
                );
              } else {
                await linkUserToInstitutionService(
                  selectedUser,
                  formValues.existingInstitution,
                );
              }
            } catch {
              if (isNewInstitution) {
                messageApi?.error("Error creating institution");
              } else {
                messageApi?.error("Error linking user to institution");
              }
            } finally {
              form.resetFields();
              setSelectedUser(null);
              fetchUsers();
            }
          }
        }}
        onCancel={() => {
          setOpen(false);
          form.resetFields();
          setSelectedUser(null);
        }}
      >
        <Form form={form} layout="vertical" name="institution_form">
          <Form.Item>
            <Button
              type="link"
              onClick={() => setIsNewInstitution(!isNewInstitution)}
            >
              {isNewInstitution
                ? "Select Existing Institution"
                : "Create New Institution"}
            </Button>
          </Form.Item>

          {isNewInstitution ? (
            <>
              <Form.Item
                name="newInstitution"
                label="New Institution Name"
                rules={[
                  {
                    required: true,
                    message: "Please input institution name!",
                  },
                ]}
              >
                <Input placeholder="Enter institution name" />
              </Form.Item>
              <Form.Item
                name="email"
                label="Institution Email"
                rules={[
                  {
                    required: true,
                    message: "Please input institution email!",
                  },
                  { type: "email", message: "Please enter a valid email!" },
                ]}
              >
                <Input placeholder="Enter institution email" />
              </Form.Item>
            </>
          ) : (
            <Form.Item
              name="existingInstitution"
              label="Select Institution"
              rules={[
                { required: true, message: "Please select an institution!" },
              ]}
            >
              <Select
                placeholder="Select an institution"
                options={institutions}
              />
            </Form.Item>
          )}
        </Form>
      </Modal>
    </>
  );
};

export default UsersTable;
