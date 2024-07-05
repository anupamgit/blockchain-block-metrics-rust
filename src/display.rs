//! This file defines the display_metrics_table function which formats the collected metrics
//! into an HTML table that can be displayed on a webpage.

use crate::transform::Metrics;

pub fn display_metrics_table(metrics: Vec<Metrics>) -> String {
    let mut table = String::from("<html><head><style>table {width: 100%; border-collapse: collapse;} th, td {border: 1px solid #ddd; padding: 8px;} th {background-color: #f2f2f2; text-align: left;} </style></head><body><h1>Blockchain Metrics</h1><table><tr><th>Blockchain</th><th>Block Number</th><th>Timestamp</th><th>Gas Used</th><th>Transactions Count</th><th>Block Size</th><th>Transaction Fees</th><th>Base Fee Per Gas</th></tr>");

    for metric in metrics {
        table.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            metric.blockchain,
            metric.block_number,
            metric.timestamp,
            metric.gas_used,
            metric.transaction_count,
            metric.block_size,
            metric.transaction_fees,
            metric.base_fee_per_gas
        ));
    }

    let legend = "<h2>Legend</h2><ul><li><strong>Gas Used</strong>: Total gas used per block (in units of gas)</li><li><strong>Transaction Count</strong>: Number of transactions in the block</li><li><strong>Block Size</strong>: Size of the block (in bytes)</li><li><strong>Transaction Fees</strong>: Total fees for transactions in the block (in Ether/Token)</li><li><strong>Base Fee Per Gas</strong>: Minimum gas price per unit of gas (in Wei)</li></ul></body></html>";

    table.push_str(legend);

    table.push_str("</table></body></html>"); // Ensure the table is closed before the legend

    table
}
