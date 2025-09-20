import { expect } from 'chai';
import { parseXBRL } from '../xbrlParser';

describe('parseXBRL', () => {
  const mockXBRL = `<?xml version="1.0" encoding="UTF-8"?>
<xbrl xmlns:dei="http://xbrl.sec.gov/dei/2023" xmlns:us-gaap="http://fasb.org/us-gaap/2023">
  <context id="AsOf_2024-12-31">
    <period><instant>2024-12-31</instant></period>
  </context>
  <context id="Period_2024">
    <period>
      <startDate>2024-01-01</startDate>
      <endDate>2024-12-31</endDate>
    </period>
  </context>
  
  <dei:EntityRegistrantName contextRef="AsOf_2024-12-31">ABC Manufacturing Corp</dei:EntityRegistrantName>
  <us-gaap:Assets contextRef="AsOf_2024-12-31" unitRef="USD">1505000</us-gaap:Assets>
  <us-gaap:Liabilities contextRef="AsOf_2024-12-31" unitRef="USD">637000</us-gaap:Liabilities>
  <us-gaap:StockholdersEquity contextRef="AsOf_2024-12-31" unitRef="USD">868000</us-gaap:StockholdersEquity>
  <us-gaap:GrossProfit contextRef="Period_2024" unitRef="USD">800000</us-gaap:GrossProfit>
  <us-gaap:NetIncomeLoss contextRef="Period_2024" unitRef="USD">141000</us-gaap:NetIncomeLoss>
  <us-gaap:OperatingIncomeLoss contextRef="Period_2024" unitRef="USD">230000</us-gaap:OperatingIncomeLoss>
</xbrl>`;

  it('should parse company name correctly', () => {
    const result = parseXBRL(mockXBRL);
    expect(result.companyName).to.equal('ABC Manufacturing Corp');
  });

  it('should parse year and quarter from date', () => {
    const result = parseXBRL(mockXBRL);
    expect(result.year).to.equal(2024);
    expect(result.quarter).to.equal(4);
  });

  it('should parse financial values correctly', () => {
    const result = parseXBRL(mockXBRL);
    expect(result.totalAssets).to.equal(1505000);
    expect(result.totalLiabilities).to.equal(637000);
    expect(result.totalEquity).to.equal(868000);
    expect(result.grossProfit).to.equal(800000);
    expect(result.netProfit).to.equal(141000);
    expect(result.operatingProfit).to.equal(230000);
  });

  it('should calculate total liabilities and equity', () => {
    const result = parseXBRL(mockXBRL);
    expect(result.totalLiabilitiesAndEquity).to.equal(1505000);
  });

  it('should return undefined for missing values', () => {
    const emptyXBRL = '<xbrl></xbrl>';
    const result = parseXBRL(emptyXBRL);
    expect(result.companyName).to.equal(undefined);
    expect(result.totalAssets).to.equal(undefined);
  });
});