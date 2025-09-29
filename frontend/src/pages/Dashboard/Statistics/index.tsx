import formatter from "@/components/shared/CountUp";
import getCycleMonitoringService from "@/services/admin/getCycleMonitoring.service";
import getStorageInfoService from "@/services/admin/getStorageInfo.service";
import { useAppSelector } from "@/store/hooks";
import {
  BankOutlined,
  DatabaseOutlined,
  FileTextOutlined,
  UserOutlined,
  WalletOutlined,
} from "@ant-design/icons";
import { Card, Col, Row, Statistic } from "antd";
import { useEffect, useState, type FC } from "react";

const Statistics: FC = () => {
  const [users, setUsers] = useState(0);
  const [documents, setDocuments] = useState(0);
  const [institutions, setInstitutions] = useState(0);
  const [cycles, setCycles] = useState(0);
  const [memory, setMemory] = useState(0);
  const [loading, setLoading] = useState(false);
  const { messageApi } = useAppSelector((state) => state.message);

  useEffect(() => {
    const fetchAdminData = async () => {
      setLoading(true);
      try {
        const [storageInfo, cycleMonitoring] = await Promise.all([
          getStorageInfoService(),
          getCycleMonitoringService(),
        ]);

        const [, docsCount, instsCount, usersCount] = storageInfo;
        setDocuments(parseInt(docsCount.split(": ")[1]));
        setInstitutions(parseInt(instsCount.split(": ")[1]));
        setUsers(parseInt(usersCount.split(": ")[1]));
        setCycles(Number(cycleMonitoring.current_balance));
        setMemory(
          Math.round(Number(cycleMonitoring.memory_size_bytes) / (1024 * 1024)),
        );
      } catch {
        messageApi?.error("Error fetching data");
      } finally {
        setLoading(false);
      }
    };

    fetchAdminData();
  }, []);
  return (
    <>
      <Row gutter={[16, 16]}>
        <Col xs={24} sm={12} md={8} lg={8} xl={8}>
          <Card variant="borderless">
            <Statistic
              title={
                <div className="statistic-title text-lg">
                  <UserOutlined /> <span>Users</span>
                </div>
              }
              value={users}
              formatter={formatter}
              loading={loading}
              // valueStyle={{ color: "#3f8600" }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8} lg={8} xl={8}>
          <Card variant="borderless">
            <Statistic
              title={
                <div className="statistic-title text-lg">
                  <FileTextOutlined /> <span>Documents</span>
                </div>
              }
              value={documents}
              formatter={formatter}
              loading={loading}
              // valueStyle={{ color: "#cf1322" }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8} lg={8} xl={8}>
          <Card variant="borderless">
            <Statistic
              title={
                <div className="statistic-title text-lg">
                  <BankOutlined /> <span>Institutions</span>
                </div>
              }
              value={institutions}
              formatter={formatter}
              loading={loading}
              // valueStyle={{ color: "#cf1322" }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8} lg={8} xl={8}>
          <Card variant="borderless">
            <Statistic
              title={
                <div className="statistic-title text-lg">
                  <WalletOutlined /> <span>Cycles</span>
                </div>
              }
              value={cycles}
              formatter={formatter}
              loading={loading}
              // valueStyle={{ color: "#3f8600" }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={8} lg={8} xl={8}>
          <Card variant="borderless">
            <Statistic
              title={
                <div className="statistic-title text-lg">
                  <DatabaseOutlined /> <span>Memory (MB)</span>
                </div>
              }
              value={memory}
              formatter={formatter}
              loading={loading}
              // valueStyle={{ color: "#cf1322" }}
            />
          </Card>
        </Col>
      </Row>
    </>
  );
};

export default Statistics;
