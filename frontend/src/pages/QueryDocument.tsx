import DocumentFilter from "@/components/shared/DocumentFilter";
import DocumentList from "@/components/shared/DocumentList";
import queryDocuments, {
  type QueryDocumentsFilters,
  type QueryDocumentsResult,
} from "@/services/documents/queryDocuments";
import { Alert, Divider, Spin } from "antd";
import React, { useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router";

const QueryDocument: React.FC = () => {
  const navigate = useNavigate();

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [results, setResults] = useState<QueryDocumentsResult | null>(null);
  const [currentFilters, setCurrentFilters] = useState<QueryDocumentsFilters>(
    {},
  );
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);

  const handleFilter = useCallback(
    async (filters: QueryDocumentsFilters) => {
      setLoading(true);
      setError(null);
      setCurrentFilters(filters);
      setCurrentPage(1);

      try {
        const queryFilters = {
          ...filters,
          offset: BigInt(0),
          limit: BigInt(pageSize),
        };

        const result = await queryDocuments(queryFilters);
        setResults(result);
      } catch (err) {
        console.error("Failed to query documents:", err);
        setError("Failed to query documents. Please try again.");
        setResults(null);
      } finally {
        setLoading(false);
      }
    },
    [pageSize],
  );

  // Load initial data on mount
  useEffect(() => {
    handleFilter({});
  }, [handleFilter]);

  const handlePageChange = useCallback(
    async (page: number, newPageSize: number) => {
      setCurrentPage(page);
      setPageSize(newPageSize);
      setLoading(true);
      setError(null);

      try {
        const queryFilters = {
          ...currentFilters,
          offset: BigInt((page - 1) * newPageSize),
          limit: BigInt(newPageSize),
        };

        const result = await queryDocuments(queryFilters);
        setResults(result);
      } catch (err) {
        console.error("Failed to load page:", err);
        setError("Failed to load page. Please try again.");
      } finally {
        setLoading(false);
      }
    },
    [currentFilters],
  );

  const handleViewDocument = useCallback(
    (documentId: string) => {
      navigate(`/document/${documentId}/view`);
    },
    [navigate],
  );

  return (
    <div className="min-h-screen bg-gray-50 py-8 px-4 sm:px-6 lg:px-8">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900">Query Documents</h1>
          <p className="mt-2 text-gray-600">
            Search and filter documents using advanced criteria
          </p>
        </div>

        <div className="mb-8">
          <DocumentFilter onFilter={handleFilter} loading={loading} />
        </div>

        <Divider />

        {error && (
          <Alert
            message="Error"
            description={error}
            type="error"
            showIcon
            className="mb-6"
          />
        )}

        {loading && !results && (
          <div className="text-center py-12">
            <Spin size="large" />
            <p className="mt-4 text-gray-600">Loading documents...</p>
          </div>
        )}

        {results && (
          <div>
            <div className="mb-4">
              <p className="text-gray-600">
                Found {results.total_count} document
                {results.total_count !== 1 ? "s" : ""}
              </p>
            </div>
            <DocumentList
              documents={results.documents}
              totalCount={results.total_count}
              loading={loading}
              currentPage={currentPage}
              pageSize={pageSize}
              onPageChange={handlePageChange}
              onViewDocument={handleViewDocument}
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default QueryDocument;
