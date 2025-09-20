export interface XBRLData {
  companyName?: string;
  year?: number;
  quarter?: number;
  totalAssets?: number;
  totalLiabilities?: number;
  totalEquity?: number;
  totalLiabilitiesAndEquity?: number;
  grossProfit?: number;
  netProfit?: number;
  operatingProfit?: number;
  profitBeforeTax?: number;
  ebitda?: number;
}

export const parseXBRL = (xmlContent: string): XBRLData => {
  const parser = new DOMParser();
  const xmlDoc = parser.parseFromString(xmlContent, 'text/xml');
  
  const getValue = (tagName: string, contextRef?: string): number | undefined => {
    const elements = xmlDoc.getElementsByTagName(tagName);
    for (let i = 0; i < elements.length; i++) {
      const element = elements[i];
      if (!contextRef || element.getAttribute('contextRef') === contextRef) {
        const value = element.textContent?.trim();
        return value ? parseFloat(value) : undefined;
      }
    }
    return undefined;
  };

  const getTextValue = (tagName: string): string | undefined => {
    const element = xmlDoc.getElementsByTagName(tagName)[0];
    return element?.textContent?.trim();
  };

  // Extract year from period context
  const getYearFromContext = (): number | undefined => {
    const endDateElement = xmlDoc.querySelector('period endDate, period instant');
    if (endDateElement) {
      const dateStr = endDateElement.textContent?.trim();
      if (dateStr) {
        return new Date(dateStr).getFullYear();
      }
    }
    return undefined;
  };

  // Calculate quarter from end date
  const getQuarterFromContext = (): number | undefined => {
    const endDateElement = xmlDoc.querySelector('period endDate, period instant');
    if (endDateElement) {
      const dateStr = endDateElement.textContent?.trim();
      if (dateStr) {
        const month = new Date(dateStr).getMonth() + 1;
        return Math.ceil(month / 3);
      }
    }
    return undefined;
  };

  return {
    companyName: getTextValue('dei:EntityRegistrantName'),
    year: getYearFromContext(),
    quarter: getQuarterFromContext(),
    totalAssets: getValue('us-gaap:Assets', 'AsOf_2024-12-31'),
    totalLiabilities: getValue('us-gaap:Liabilities', 'AsOf_2024-12-31'),
    totalEquity: getValue('us-gaap:StockholdersEquity', 'AsOf_2024-12-31'),
    totalLiabilitiesAndEquity: getValue('us-gaap:Liabilities', 'AsOf_2024-12-31') && getValue('us-gaap:StockholdersEquity', 'AsOf_2024-12-31') 
      ? (getValue('us-gaap:Liabilities', 'AsOf_2024-12-31')! + getValue('us-gaap:StockholdersEquity', 'AsOf_2024-12-31')!)
      : undefined,
    grossProfit: getValue('us-gaap:GrossProfit', 'Period_2024'),
    netProfit: getValue('us-gaap:NetIncomeLoss', 'Period_2024'),
    operatingProfit: getValue('us-gaap:OperatingIncomeLoss', 'Period_2024'),
    profitBeforeTax: getValue('us-gaap:IncomeLossFromContinuingOperationsBeforeIncomeTaxesExtraordinaryItemsNoncontrollingInterest', 'Period_2024'),
    // EBITDA calculation: Operating Income + Depreciation (if available)
    ebitda: getValue('us-gaap:OperatingIncomeLoss', 'Period_2024'),
  };
};