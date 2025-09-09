//! # Subclient Code Generation
//!
//! This build script generates specialized client structs (subclients) for Privy API resources
//! based on configuration from `stainless.yml` and the OpenAPI specification.
//!
//! ## Generation Process
//!
//! 1. **Base Code Generation**: Uses progenitor to generate the core `Client` from `openapi.json`
//!    and writes it to `$OUT_DIR/codegen.rs`.
//!
//! 2. **Configuration Parsing**: Reads `stainless.yml` to extract resource structure, including:
//!    - Resource names (e.g., "wallets", "apps")
//!    - Method mappings (e.g., "list" -> "get /v1/wallets")
//!    - Nested subresources (e.g., "wallets.rpc")
//!    - Method filtering (excludes methods starting with '_')
//!
//! 3. **Method Signature Extraction**: Parses the generated progenitor code to extract method
//!    signatures from the `Client` impl block, preserving parameter types, return types, and
//!    documentation comments.
//!
//! 4. **Subclient Generation**: Creates specialized client structs for each resource:
//!    - `WalletsClient`, `AppsClient`, etc. for top-level resources
//!    - `WalletsRpcClient` for nested subresources
//!    - Each subclient wraps the base `Client` and delegates to appropriate methods
//!    - Method names are mapped from YAML config to OpenAPI operationId using snake_case
//!
//! 5. **Main Client Extension**: Generates accessor methods on `PrivyClient` to access each
//!    subclient (e.g., `client.wallets()` returns a `WalletsClient`).
//!
//! The final generated code is written to `$OUT_DIR/subclients.rs` and included in the main
//! library, providing a structured, resource-oriented API surface.

use std::{collections::HashMap, fs};

use heck::{ToPascalCase, ToSnakeCase};
use progenitor::GenerationSettings;
use quote::quote;
use serde_yaml::Value;
use syn::{File, Item, ItemImpl, Signature};

/// Configuration for a subclient resource from stainless.yml
#[derive(Debug, Clone)]
struct ResourceConfig {
    name: String,
    methods: Vec<MethodConfig>,
    subresources: Vec<ResourceConfig>,
}

/// Configuration for a method within a resource
#[derive(Debug, Clone)]
struct MethodConfig {
    name: String,
    endpoint: String,
    should_generate: bool, // false for methods starting with '_'
}

/// Information extracted from the generated progenitor code
#[derive(Debug, Clone)]
struct GeneratedMethod {
    #[allow(dead_code)]
    name: String,
    signature: Signature,
    doc_comment: Option<String>,
}

fn main() {
    println!("cargo:rerun-if-changed=openapi.json");
    println!("cargo:rerun-if-changed=stainless.yml");

    // Step 1: Generate the base progenitor code
    let openapi_spec = load_openapi_spec();
    let mut generator = progenitor::Generator::new(
        GenerationSettings::default().with_inner_type(quote! {crate::middleware::MiddlewareState}),
    );
    let tokens = generator.generate_tokens(&openapi_spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    // Write the base generated code
    let mut out_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    std::fs::write(&out_file, &content).unwrap();

    // Step 2: Parse the stainless.yml configuration
    let resource_configs = parse_stainless_config();

    // Step 3: Parse the generated code to extract method signatures
    let generated_methods = parse_generated_code(&ast);

    // Step 4: Generate the subclient code
    let subclient_code =
        generate_subclient_code(&resource_configs, &generated_methods, &openapi_spec);

    // Step 5: Write the subclient code to a separate file
    let mut subclient_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
    subclient_file.push("subclients.rs");
    std::fs::write(subclient_file, subclient_code).unwrap();
}

/// Load and parse the OpenAPI specification
fn load_openapi_spec() -> openapiv3::OpenAPI {
    let file = std::fs::File::open("openapi.json").unwrap();
    serde_json::from_reader(file).unwrap()
}

/// Parse the stainless.yml file to extract resource configuration
fn parse_stainless_config() -> Vec<ResourceConfig> {
    let content = fs::read_to_string("stainless.yml").unwrap();
    let yaml: Value = serde_yaml::from_str(&content).unwrap();

    let mut resources = Vec::new();

    if let Some(resources_map) = yaml.get("resources").and_then(|r| r.as_mapping()) {
        for (resource_name, resource_config) in resources_map {
            if let Some(name) = resource_name.as_str() {
                let resource = parse_resource_config(name, resource_config);
                resources.push(resource);
            }
        }
    }

    resources
}

/// Parse a single resource configuration from the YAML
fn parse_resource_config(name: &str, config: &Value) -> ResourceConfig {
    let mut methods = Vec::new();
    let mut subresources = Vec::new();

    // Parse methods
    if let Some(methods_map) = config.get("methods").and_then(|m| m.as_mapping()) {
        for (method_name, method_config) in methods_map {
            if let Some(method_name_str) = method_name.as_str() {
                let endpoint = match method_config {
                    Value::String(s) => s.clone(),
                    Value::Mapping(map) => map
                        .get("endpoint")
                        .and_then(|e| e.as_str())
                        .unwrap_or("")
                        .to_string(),
                    _ => continue,
                };

                // Skip methods that start with underscore unless they're the Rust version
                let should_generate = !method_name_str.starts_with('_');

                methods.push(MethodConfig {
                    name: method_name_str.to_string(),
                    endpoint,
                    should_generate,
                });
            }
        }
    }

    // Parse subresources
    if let Some(subresources_map) = config.get("subresources").and_then(|s| s.as_mapping()) {
        for (subresource_name, subresource_config) in subresources_map {
            if let Some(subresource_name_str) = subresource_name.as_str() {
                let subresource = parse_resource_config(subresource_name_str, subresource_config);
                subresources.push(subresource);
            }
        }
    }

    ResourceConfig {
        name: name.to_string(),
        methods,
        subresources,
    }
}

/// Parse the generated progenitor code to extract method signatures
fn parse_generated_code(ast: &File) -> HashMap<String, GeneratedMethod> {
    let mut methods = HashMap::new();

    // Look for the Client impl block
    for item in &ast.items {
        if let Item::Impl(impl_block) = item {
            if let Some((_, _path, _)) = &impl_block.trait_ {
                continue; // Skip trait implementations
            }

            // Check if this is the Client impl
            if let syn::Type::Path(type_path) = &*impl_block.self_ty {
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "Client" {
                        extract_methods_from_impl(impl_block, &mut methods);
                    }
                }
            }
        }
    }

    methods
}

/// Extract method information from an impl block
fn extract_methods_from_impl(
    impl_block: &ItemImpl,
    methods: &mut HashMap<String, GeneratedMethod>,
) {
    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = method.sig.ident.to_string();

            // Extract doc comments
            let doc_comment = extract_doc_comments(&method.attrs);

            methods.insert(
                method_name.clone(),
                GeneratedMethod {
                    name: method_name,
                    signature: method.sig.clone(),
                    doc_comment,
                },
            );
        }
    }
}

/// Extract documentation comments from attributes
fn extract_doc_comments(attrs: &[syn::Attribute]) -> Option<String> {
    let mut doc_lines = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta_name_value) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &meta_name_value.value
                {
                    doc_lines.push(lit_str.value());
                }
            }
        }
    }

    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join("\n"))
    }
}

/// Generate the subclient code based on the configuration and parsed methods
fn generate_subclient_code(
    resources: &[ResourceConfig],
    generated_methods: &HashMap<String, GeneratedMethod>,
    openapi_spec: &openapiv3::OpenAPI,
) -> String {
    let mut code_parts = Vec::new();

    // Add imports
    code_parts.push(quote! {
        use crate::generated::{Client, Error, ResponseValue, ByteStream, types};
    });

    // Generate code for each resource
    for resource in resources {
        let subclient_code = generate_resource_code(resource, generated_methods, "", openapi_spec);
        code_parts.push(subclient_code);
    }

    // Generate the main PrivyClient extension
    let main_client_extension = generate_main_client_extension(resources);
    code_parts.push(main_client_extension);

    // Combine all code parts
    let combined = quote! {
        #(#code_parts)*
    };

    prettyplease::unparse(&syn::parse2(combined).unwrap())
}

/// Generate code for a single resource (including subresources)
fn generate_resource_code(
    resource: &ResourceConfig,
    generated_methods: &HashMap<String, GeneratedMethod>,
    parent_path: &str,
    openapi_spec: &openapiv3::OpenAPI,
) -> proc_macro2::TokenStream {
    let resource_name = &resource.name;
    let client_name = if parent_path.is_empty() {
        format!("{}Client", resource_name.to_pascal_case())
    } else {
        format!(
            "{}{}Client",
            parent_path.to_pascal_case(),
            resource_name.to_pascal_case()
        )
    };
    let client_ident = syn::Ident::new(&client_name, proc_macro2::Span::call_site());

    // Generate the struct
    let struct_msg = format!("Client for {} operations", resource_name);
    let struct_def = quote! {
        #[doc = #struct_msg]
        #[derive(Clone, Debug)]
        pub struct #client_ident {
            client: Client,
        }
    };

    // Generate the impl block
    let mut impl_methods = Vec::new();

    // Add methods for this resource
    for method in &resource.methods {
        if method.should_generate {
            if let Some(method_impl) =
                generate_method_impl(method, generated_methods, resource_name, openapi_spec)
            {
                impl_methods.push(method_impl);
            }
        }
    }

    // Add accessor methods for subresources
    for subresource in &resource.subresources {
        let subresource_method = generate_subresource_accessor(&subresource.name, resource_name);
        impl_methods.push(subresource_method);
    }

    let impl_block = quote! {
        impl #client_ident {
            /// Create a new client instance
            pub fn new(client: Client) -> Self {
                Self { client }
            }

            #(#impl_methods)*
        }
    };

    // Generate code for subresources
    let mut subresource_code = Vec::new();
    for subresource in &resource.subresources {
        let sub_path = if parent_path.is_empty() {
            resource_name.clone()
        } else {
            format!("{}.{}", parent_path, resource_name)
        };
        let sub_code =
            generate_resource_code(subresource, generated_methods, &sub_path, openapi_spec);
        subresource_code.push(sub_code);
    }

    quote! {
        #struct_def
        #impl_block
        #(#subresource_code)*
    }
}

/// Generate a method implementation for a subclient
fn generate_method_impl(
    method: &MethodConfig,
    generated_methods: &HashMap<String, GeneratedMethod>,
    _resource_name: &str,
    openapi_spec: &openapiv3::OpenAPI,
) -> Option<proc_macro2::TokenStream> {
    // Map the method name to the actual generated method name
    let generated_method_name = map_method_name(method, openapi_spec)?;

    if let Some(generated_method) = generated_methods.get(&generated_method_name) {
        let method_name = syn::Ident::new(&method.name, proc_macro2::Span::call_site());
        let generated_method_ident =
            syn::Ident::new(&generated_method_name, proc_macro2::Span::call_site());

        // Clone the signature and modify it
        let mut sig = generated_method.signature.clone();
        sig.ident = method_name;

        // Update self parameter to use our client
        if let Some(syn::FnArg::Receiver(receiver)) = sig.inputs.first_mut() {
            receiver.mutability = None; // Remove mutability if present
        }

        // Extract parameter names for the delegation call
        let mut param_names = Vec::new();
        for input in sig.inputs.iter().skip(1) {
            // Skip self
            if let syn::FnArg::Typed(typed) = input {
                if let syn::Pat::Ident(ident) = &*typed.pat {
                    param_names.push(&ident.ident);
                }
            }
        }

        // Generate doc comment
        let doc_comment = if let Some(doc) = &generated_method.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            quote! {
                #(
                    #[doc = #doc_lines]
                )*
            }
        } else {
            quote! {}
        };

        // Check if the original method is async
        let call_expr = if sig.asyncness.is_some() {
            quote! { self.client.#generated_method_ident(#(#param_names),*).await }
        } else {
            quote! { self.client.#generated_method_ident(#(#param_names),*) }
        };

        Some(quote! {
            #doc_comment
            pub #sig {
                #call_expr
            }
        })
    } else {
        None
    }
}

/// Generate an accessor method for a subresource
fn generate_subresource_accessor(
    subresource_name: &str,
    parent_name: &str,
) -> proc_macro2::TokenStream {
    let method_name = syn::Ident::new(subresource_name, proc_macro2::Span::call_site());
    let client_name = format!(
        "{}{}Client",
        parent_name.to_pascal_case(),
        subresource_name.to_pascal_case()
    );
    let client_ident = syn::Ident::new(&client_name, proc_macro2::Span::call_site());

    let msg = format!("Access the {} subclient", subresource_name);
    quote! {
        #[doc = #msg]
        pub fn #method_name(&self) -> #client_ident {
            #client_ident::new(self.client.clone())
        }
    }
}

/// Generate the main PrivyClient extension with resource accessors
fn generate_main_client_extension(resources: &[ResourceConfig]) -> proc_macro2::TokenStream {
    let mut accessor_methods = Vec::new();

    for resource in resources {
        let method_name = syn::Ident::new(&resource.name, proc_macro2::Span::call_site());
        let client_name = format!("{}Client", resource.name.to_pascal_case());
        let client_ident = syn::Ident::new(&client_name, proc_macro2::Span::call_site());

        let msg = format!("Access the {} client", resource.name);
        accessor_methods.push(quote! {
            #[doc = #msg]
            pub fn #method_name(&self) -> #client_ident {
                #client_ident::new(self.client.clone())
            }
        });
    }

    quote! {
        impl crate::client::PrivyClient {
            #(#accessor_methods)*
        }
    }
}

/// Map a method name from the YAML to the actual generated method name using OpenAPI operationId
fn map_method_name(
    method_config: &MethodConfig,
    openapi_spec: &openapiv3::OpenAPI,
) -> Option<String> {
    // Parse the endpoint from the YAML (e.g., "get /v1/wallets" or "post /v1/wallets/{wallet_id}")
    let Some((http_method, path)) = method_config.endpoint.split_once(' ') else {
        return Some(method_config.name.clone());
    };

    // Find the matching operation in the OpenAPI spec
    let Some(openapiv3::ReferenceOr::Item(path_item)) = openapi_spec.paths.paths.get(path) else {
        return Some(method_config.name.clone());
    };

    let operation = match http_method.to_lowercase().as_str() {
        "get" => &path_item.get,
        "post" => &path_item.post,
        "put" => &path_item.put,
        "delete" => &path_item.delete,
        "patch" => &path_item.patch,
        _ => return Some(method_config.name.clone()),
    };

    if let Some(operation_id) = operation.as_ref().and_then(|o| o.operation_id.as_ref()) {
        Some(operation_id.to_snake_case())
    } else {
        Some(method_config.name.clone())
    }
}
