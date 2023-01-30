<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateBusinessBankAccount</name>
   <tag></tag>
   <elementGuidId>4eae7ba2-b903-4611-b17a-8181909f1464</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;business_account_id\&quot;: \&quot;${business_account_id}\&quot;,\n  \&quot;bank_id\&quot;: \&quot;${bank_id}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;name\&quot;: \&quot;${name}\&quot;,\n  \&quot;account_holder_name\&quot;: \&quot;${account_holder_name}\&quot;,\n  \&quot;iban\&quot;: \&quot;${iban}\&quot;,\n  \&quot;account_no\&quot;: \&quot;${account_no}\&quot;,\n  \&quot;swift_code\&quot;: \&quot;${swift_code}\&quot;,\n  \&quot;branch_code\&quot;: \&quot;${branch_code}\&quot;,\n  \&quot;is_active\&quot;: \&quot;${is_active}\&quot;,\n  \&quot;is_deleted\&quot;: \&quot;${is_deleted}\&quot;,\n  \&quot;bank_name\&quot;: \&quot;${bank_name}\&quot;,\n  \&quot;created_date_utc\&quot;: \&quot;${created_date_utc}\&quot;,\n  \&quot;updated_date_utc\&quot;: \&quot;${updated_date_utc}\&quot;\n}&quot;,
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
      <webElementGuid>595c50e4-e02c-46d7-b136-1b2b6c9d9355</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>cc962f7d-db18-494e-9afc-b2109dd816d0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/CreateBusinessBankAccount</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>e846c6d8-7ab4-4f58-b931-83f8e247af5f</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>c6acbcb6-ee90-4f5c-ab5f-1c5823490712</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>014c86fb-c16b-4f74-b480-98e8f42ca5cd</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3aea3a95-527a-4c2f-bd18-ee4b339837b3</id>
      <masked>false</masked>
      <name>business_account_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7fecbae9-f3a4-4669-a12f-8666a4152737</id>
      <masked>false</masked>
      <name>bank_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9fedb32e-5e1d-4681-a86e-2e707507c4e2</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b06f5b38-cd15-462b-956d-d17b15fe2388</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bc305c6b-a35b-4997-bb29-153424e55da9</id>
      <masked>false</masked>
      <name>account_no</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f04b594-ae35-452a-90f5-941f2f38ad31</id>
      <masked>false</masked>
      <name>iban</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2a080494-f019-4041-bc79-2a891ad07bb8</id>
      <masked>false</masked>
      <name>swift_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5cbe304f-6f96-4e16-be97-e40bbf4d13e3</id>
      <masked>false</masked>
      <name>branch_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f4a52173-3ac9-4d45-b123-8d5e352f0ab6</id>
      <masked>false</masked>
      <name>is_active</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d7a67862-94f2-4475-94f4-14b5e35985f2</id>
      <masked>false</masked>
      <name>is_deleted</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5c5ccfc3-bdda-4765-8149-1ffe40dd0fb9</id>
      <masked>false</masked>
      <name>bank_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fcdf7416-152e-4c66-9813-78b9ddd970e2</id>
      <masked>false</masked>
      <name>created_date_utc</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6d7bea29-100a-4335-8b93-c1763a08cbf9</id>
      <masked>false</masked>
      <name>updated_date_utc</name>
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


println(&quot;CreateBusinessBankAccount request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyElementPropertyValue(response, 'status', 0)
  
import org.json.JSONObject;

JSONObject json = new JSONObject(response.getBodyContent().getText());

println(&quot;CreateBusinessBankAccount response body\n&quot;+json.toString(4));
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
