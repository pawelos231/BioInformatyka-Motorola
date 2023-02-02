use std::fmt::{Display, Write};

use crate::{Acids, Bases, Codon, Nucleotide, Protein};

#[derive(Default, Clone)]
pub struct AminoString {
	codons: Vec<Codon>,
}

impl AminoString {
	pub const fn from(codons: Vec<Codon>) -> Self {
		Self { codons }
	}

	pub fn push(&mut self, codon: Codon) {
		self.codons.push(codon);
	}

	pub fn len(&self) -> usize {
		self.codons.len()
	}

	pub fn is_empty(&self) -> bool {
		self.codons.len() == 0
	}

	pub fn get_codons(&self) -> &Vec<Codon> {
		&self.codons
	}

	pub fn get_codons_mut(&mut self) -> &mut Vec<Codon> {
		&mut self.codons
	}

	pub fn get_first(&self) -> Codon {
		self.codons[0]
	}

	pub fn get_last(&self) -> Codon {
		*self.codons.last().unwrap()
	}

	// physical properties

	pub fn get_mass(&self) -> f32 {
		let codon_len = self.codons.len() as f32;
		let sum = crate::ALPHA_MASS * codon_len + crate::H2_MASS;

		let final_mass: f32 = sum
			+ self
				.codons
				.iter()
				.map(|x| x.get_acid().map(|x| x.sc_mass).unwrap_or(0f32))
				.sum::<f32>();
		final_mass
	}

	pub fn get_isoletric_point(&self) {
		let _bases = Bases::init_bases(&self.get_last().get_acid().unwrap().pk2);
		let _acids = Acids::init_acids(&self.get_first().get_acid().unwrap().pk1);
		for ph in (0..1400).map(|x| x as f64 * 0.01) {
			println!("Index {ph}");
		}
		println!("{}", &self.get_first().get_acid().unwrap());
	}

	pub fn net_charge(_acids: Acids, _bases: Bases, _ph: f64) -> f32 {
		let _c = 0.0;

		0.5
	}

	pub fn add_signum(hydrophobicity: f32) -> String {
		if hydrophobicity > 0.0 {
			format!("+{hydrophobicity}")
		} else {
			hydrophobicity.to_string()
		}
	}

	pub fn get_phob(&self, n: usize) -> f32 {
		let hydrophobicity = 7.9;
		let final_hydrophobicity: f32 = hydrophobicity
			+ self
				.codons
				.iter()
				.map(|x| x.get_acid().map(|x| x.sc_hbob).unwrap_or(0f32))
				.take(n)
				.sum::<f32>();

		final_hydrophobicity

		// let mut return_val = AminoString::add_signum(final_hydrophobicity);
		// return_val.push_str("Kcal * mol⁻¹");
		// return_val
	}

	pub fn get_polarity(&self) -> f32 {
		0.5
	}

	pub fn parse(source: &str) -> Vec<Self> {
		let mut temp = [Nucleotide::A, Nucleotide::A, Nucleotide::A];
		let mut temp_idx = 0;

		let mut res = Vec::new();

		for index in 0..3.min(source.len()) {
			let mut codons = Vec::with_capacity(source.len() / 3);
			source
				.chars()
				.skip(index)
				.filter(|x| *x != ' ')
				.for_each(|x| {
					temp[temp_idx] = Nucleotide::parse(x).unwrap();
					temp_idx += 1;
					if temp_idx == 3 {
						codons.push(Codon::new(temp[0], temp[1], temp[2]));
						temp_idx = 0;
					}
				});
			res.push(AminoString::from(codons));
			temp_idx = 0;
		}
		res
	}

	pub fn get_proteins(&self) -> Vec<Protein> {
		let mut result = Vec::new();

		let mut current = Vec::with_capacity(30000);
		let mut protein = false;
		for codon in &self.codons {
			let acid = codon.get_acid_shorthand();

			if acid == Codon::STOP && protein {
				if !current.is_empty() {
					result.push(Protein::from(current.clone()));
					current.clear();
				}
				protein = false;
			}

			if protein {
				current.push(*codon);
			}

			if acid == Codon::START {
				protein = true;
			}
		}
		result
	}
}

impl Display for AminoString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for codon in &self.codons {
			f.write_char(codon.get_acid_shorthand())?;
		}
		Ok(())
	}
}
