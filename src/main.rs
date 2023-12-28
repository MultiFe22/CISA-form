use lopdf::Document;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let original_file = "/Users/felipevidal/Downloads/Secure Software Development Attestation Form_508c.pdf";
    let doc = Document::load(original_file).unwrap();
    let total_pages = doc.get_pages().len();

    for i in 1..=total_pages {
        // Reload the original document for each iteration
        let mut doc = Document::load(original_file).unwrap();

        // Create a vector of pages to delete, excluding the current page
        let pages_to_delete: Vec<u32> = (1..=total_pages)
            .filter(|&page| page != i)
            .map(|page| page as u32)
            .collect();

        // Delete other pages
        doc.delete_pages(&pages_to_delete);
        // Save the current page as a separate PDF
        let output_file = format!("form/output_page_{}.pdf", i);
        doc.save(&output_file).unwrap();
        
    }

    println!("Elapsed time: {:.2?}", now.elapsed());
}
