<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register_BusinessAccount</name>
   <tag></tag>
   <elementGuidId>02c52fbe-c9e7-45e3-a14f-8b68ed2aec6f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;user_email\&quot;: \&quot;${user_email}\&quot;,\n  \&quot;tax_number\&quot;:\&quot;${tax_number}\&quot;,\n  \&quot;business_type\&quot;: \&quot;${business_type}\&quot;,\n  \&quot;name\&quot;:\&quot;${name}\&quot;,\n  \&quot;alias\&quot;: \&quot;${alias}\&quot;,\n  \&quot;user_number\&quot;: \&quot;${user_number}\&quot;,\n  \&quot;user_first_name\&quot;: \&quot;${user_first_name}\&quot;,\n  \&quot;user_last_name\&quot;: \&quot;${user_last_name}\&quot;,\n  \&quot;user_phone_country_code\&quot;: \&quot;${user_phone_country_code}\&quot;,\n  \&quot;user_phone_number\&quot;: \&quot;${user_phone_number}\&quot;,\n  \&quot;tax_office\&quot;:\&quot;${tax_office}\&quot;,\n  \&quot;use_iban\&quot;: ${use_iban},\n  \&quot;contact_address\&quot;: {\n    \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n    \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address_line1\&quot;: \&quot;${address_line1}\&quot;,\n    \&quot;address_line2\&quot;: \&quot;${address_line2}\&quot;,\n    \&quot;zip_postal_code\&quot;: \&quot;${zip_postal_code}\&quot;,\n    \&quot;phone_number\&quot;: \&quot;${phone_number}\&quot;,\n    \&quot;state_province_code\&quot;: \&quot;${state_province_code}\&quot;,\n    \&quot;country_code\&quot;: \&quot;${country_code}\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>939ea504-342a-4732-98e2-171d8626b9b2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>af1de8f9-0715-466e-afc5-5f103c27a8b5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/RegisterBusinessAccount</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>05df0ec1-af81-4d4f-a0f9-137f8bfc5b82</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>28d25b89-79d0-4e2c-8d99-9bd69a702c6d</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1a4c608e-3609-499a-a1f6-5ff59eb39cd5</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>524c5a6d-fdda-4183-8deb-e97e54799e59</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cc4f5bb5-5ae0-4871-bb4e-1f12a4b3d47e</id>
      <masked>false</masked>
      <name>business_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e94cb01f-0432-4b5e-8b33-9f3e81ef710b</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f70fd954-c2f7-4671-8ff2-72bcd457d702</id>
      <masked>false</masked>
      <name>alias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1a7d01a0-dc16-4217-88d1-6c00b5ee8b4f</id>
      <masked>false</masked>
      <name>user_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>be51b87b-b1ce-4881-b670-00a9add07b7b</id>
      <masked>false</masked>
      <name>user_first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6c0b70a7-332f-435f-a31f-dd8a2699051b</id>
      <masked>false</masked>
      <name>user_last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0922339b-40fd-47dd-a6ea-c4efff098308</id>
      <masked>false</masked>
      <name>user_phone_country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>561d9cf3-403a-4a7f-a438-c2620463b451</id>
      <masked>false</masked>
      <name>user_phone_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e1bafd73-98aa-4a09-b0f7-e09e97d54a65</id>
      <masked>false</masked>
      <name>user_email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7b1e52dd-2c91-4555-ab3d-30619302b85f</id>
      <masked>false</masked>
      <name>tax_office</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0cb559ff-a19e-4fde-84e5-a62588df1ad2</id>
      <masked>false</masked>
      <name>tax_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0016fb1c-93e9-4620-b245-663840c39b4e</id>
      <masked>false</masked>
      <name>use_iban</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f511d0b5-8a68-4014-b649-080c3f8d1909</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5547faac-5fb8-42a6-8290-899e17ac5b84</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1dbbb3e7-476d-41c3-b7e9-142194812f68</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e79d4947-db43-4924-bc00-3e69f4ea155b</id>
      <masked>false</masked>
      <name>address_line1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>defcd5b3-49af-4692-b5a2-583b5f6f7f1d</id>
      <masked>false</masked>
      <name>address_line2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1c8652be-e259-4c42-a095-66041422030a</id>
      <masked>false</masked>
      <name>zip_postal_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>767335b8-93bf-45c8-80ac-bf31aafab9a7</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>894a2925-45de-4a05-a50e-8c2f2b4c282b</id>
      <masked>false</masked>
      <name>state_province_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fd6183cb-937a-4e21-a272-74c33325605d</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


GlobalVariable.responseObject = response
GlobalVariable.requestObject = request

println(&quot;RegisterBusinessAccount request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401) 
  
import org.json.JSONObject;

JSONObject json = new JSONObject(response.getBodyContent().getText()); 

println(&quot;RegisterBusinessAccount response body\n&quot;+json.toString(4));  

 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
